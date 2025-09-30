use std::{
    f64::INFINITY,
    fs::File,
    io::{self, Write},
    path::Path,
};

use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

use crate::{
    color::{Color, color, linear_to_gamma},
    hittable::Hittable,
    math::{DVec3, Point3, dvec3, point3},
    ray::Ray,
    utility::clamp,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

struct ViewportData {
    img_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
}

pub fn render<H: Hittable, P: AsRef<Path>>(
    filename: P,
    camera: &Camera,
    world: &H,
) -> Result<(), Box<dyn std::error::Error>> {
    let viewport_data = initialize(camera);

    let bar = ProgressBar::new(viewport_data.img_height as u64 * camera.img_width as u64)
        .with_finish(ProgressFinish::AbandonWithMessage("Done!".into()))
        .with_style(
            ProgressStyle::with_template(
                "{msg}: [{wide_bar:.green/cyan}] {percent}% {elapsed_precise:.dim}",
            )?
            .progress_chars("+> "),
        )
        .with_message("Rendering");

    let pixels = (0..viewport_data.img_height)
        .cartesian_product(0..camera.img_width)
        .map(|(j, i)| {
            let seed = (j * camera.img_width + i) as u64 ^ 1234;
            let mut rng = Xoshiro256Plus::seed_from_u64(seed);
            let pixel_color: Color = (0..camera.samples_per_pixel)
                .map(|_| {
                    let ray = get_ray(&mut rng, &viewport_data, i, j);
                    ray_color(&mut rng, camera.max_depth, &ray, world)
                })
                .fold(Color::ZERO, move |acc, c| acc + c);

            viewport_data.pixel_samples_scale * pixel_color
        })
        .map(|color| format_color(color))
        .progress_with(bar)
        .join("\n");

    let file = File::create(filename)?;
    let mut buf = io::BufWriter::new(file);

    // writes the header
    writeln!(
        buf,
        "P3\n{} {}\n255",
        camera.img_width, viewport_data.img_height
    )?;

    buf.write(pixels.as_bytes())?;
    buf.flush()?;

    Ok(())
}

fn initialize(camera: &Camera) -> ViewportData {
    let aspect_ratio = camera.aspect_ratio;
    let img_width = camera.img_width;

    // calculate image height, and ensure that it's at least 1.
    let img_h = (img_width as f64 / aspect_ratio) as i32;
    let img_height = if img_h < 1 { 1 } else { img_h };

    let pixel_samples_scale = 1.0 / camera.samples_per_pixel as f64;

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let center = point3(0.0, 0.0, 0.0);

    // calculate the vectors across the horizontal and down the verical viewport edges
    let viewport_u = dvec3(viewport_width, 0.0, 0.0);
    let viewport_v = dvec3(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        center - dvec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    ViewportData {
        img_height,
        pixel_samples_scale,
        center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
    }
}

fn get_ray<R: Rng>(rng: &mut R, v_data: &ViewportData, i: i32, j: i32) -> Ray {
    let offset = sample_square(rng);
    let pixel_sample = v_data.pixel00_loc
        + ((i as f64 + offset.x) * v_data.pixel_delta_u)
        + ((j as f64 + offset.y) * v_data.pixel_delta_v);
    let ray_origin = v_data.center;
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(ray_origin, ray_direction)
}

fn sample_square<R: Rng>(rng: &mut R) -> DVec3 {
    let random_double = &mut move || rng.random_range(0.0..1.0);
    dvec3(random_double() - 0.5, random_double() - 0.5, 0.0)
}

fn ray_color<H: Hittable, R: Rng>(rng: &mut R, max_depth: i32, ray: &Ray, world: &H) -> Color {
    if max_depth <= 0 {
        return Color::ZERO;
    }

    if let Some(record) = world.hit(ray, 0.001..INFINITY) {
        if let Some(scattered) = record.mat.scatter(rng, ray, &record) {
            scattered.attenuation * ray_color(rng, max_depth - 1, &scattered.ray, world)
        } else {
            Color::ZERO
        }
    } else {
        let unit_dir = ray.direction().normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
    }
}

fn format_color(c: Color) -> String {
    let r = linear_to_gamma(c.x);
    let g = linear_to_gamma(c.y);
    let b = linear_to_gamma(c.z);

    let intensity = 0.000..0.999;
    let ir = (256.0 * clamp(intensity.clone(), r)) as i32;
    let ig = (256.0 * clamp(intensity.clone(), g)) as i32;
    let ib = (256.0 * clamp(intensity, b)) as i32;

    format!("{} {} {}", ir, ig, ib)
}
