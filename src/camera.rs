use std::{
    f64::INFINITY,
    fs::File,
    io::{self, Write},
};

use indicatif::ProgressIterator;
use rand::Rng;

use crate::{
    color::{Color, color},
    hittable::Hittable,
    math::{Point3, Vec3, point3},
    ray::Ray,
    utility::clamp,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
}

struct ViewportData {
    img_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

pub fn render<H: Hittable>(camera: &Camera, world: &H) -> io::Result<()> {
    let viewport_data = initialize(camera);

    let file = File::create("image.ppm")?;
    let mut buf = io::BufWriter::new(file);

    // writes the header
    writeln!(
        buf,
        "P3\n{} {}\n255",
        camera.img_width, viewport_data.img_height
    )?;

    let mut rng = rand::rng();

    for j in (0..viewport_data.img_height).progress() {
        for i in 0..camera.img_width {
            let mut pixel_color = Color::zero();
            for _sample in 0..camera.samples_per_pixel {
                let ray = get_ray(&viewport_data, i, j);
                pixel_color += ray_color(&mut rng, &ray, world);
            }

            write_color(viewport_data.pixel_samples_scale * pixel_color, &mut buf)?;
        }
    }

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
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / img_width.into();
    let pixel_delta_v = viewport_v / img_height.into();

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
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

fn get_ray(v_data: &ViewportData, i: i32, j: i32) -> Ray {
    let offset = sample_square();
    let pixel_sample = v_data.pixel00_loc
        + ((i as f64 + offset.x()) * v_data.pixel_delta_u)
        + ((j as f64 + offset.y()) * v_data.pixel_delta_v);
    let ray_origin = v_data.center;
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(ray_origin, ray_direction)
}

fn sample_square() -> Vec3 {
    let random_double = || {
        let mut rng = rand::rng();
        rng.random_range(0.0..1.0)
    };
    Vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
}

fn ray_color<H: Hittable, R: Rng>(rng: &mut R, ray: &Ray, world: &H) -> Color {
    if let Some(record) = world.hit(ray, 0.0..INFINITY) {
        let dir = Vec3::random_on_hemisphere(rng, &record.normal);
        0.5 * ray_color(rng, &Ray::new(record.point, dir), world)
    } else {
        let unit_dir = ray.direction().norm();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
    }
}

fn write_color<W: Write>(c: Color, mut w: W) -> io::Result<()> {
    let r = c.x();
    let g = c.y();
    let b = c.z();

    let intensity = 0.000..0.999;
    let ir = (256.0 * clamp(intensity.clone(), r)) as i32;
    let ig = (256.0 * clamp(intensity.clone(), g)) as i32;
    let ib = (256.0 * clamp(intensity, b)) as i32;

    writeln!(w, "{} {} {}", ir, ig, ib)?;
    Ok(())
}
