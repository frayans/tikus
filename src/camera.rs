use std::{
    f64::INFINITY,
    fs::File,
    io::{self, Write},
};

use indicatif::ProgressIterator;

use crate::{
    color::{Color, color},
    hittable::Hittable,
    math::{Point3, Vec3, point3},
    ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
}

struct ViewportData {
    img_width: i32,
    img_height: i32,
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
        viewport_data.img_width, viewport_data.img_height
    )?;

    for j in (0..viewport_data.img_height).progress() {
        for i in 0..viewport_data.img_width {
            let pixel_center = viewport_data.pixel00_loc
                + (i as f64 * viewport_data.pixel_delta_u)
                + (j as f64 * viewport_data.pixel_delta_v);
            let ray_direction = pixel_center - viewport_data.center;
            let ray = Ray::new(viewport_data.center, ray_direction);

            let pixel_color = ray_color(&ray, world);
            write_color(pixel_color, &mut buf)?;
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
        img_width,
        img_height,
        center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
    }
}

fn ray_color<H: Hittable>(ray: &Ray, world: &H) -> Color {
    if let Some(record) = world.hit(ray, 0.0..INFINITY) {
        0.5 * (record.normal + color(1.0, 1.0, 1.0))
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

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    writeln!(w, "{} {} {}", ir, ig, ib)?;
    Ok(())
}
