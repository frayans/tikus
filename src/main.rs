use std::io;
use std::io::Write;

use tikus::math::{Point3, point3, vec3};
use tikus::{Color, Ray, color};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().mag2();
    let h = ray.direction().dot(oc);
    let c = oc.mag2() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&point3(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - vec3(0.0, 0.0, -1.0)).norm();
        0.5 * color(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_dir = ray.direction().norm();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    // calculate image height, and ensure that it's at least 1.
    let img_h = (img_width as f64 / aspect_ratio) as i32;
    let img_height = if img_h < 1 { 1 } else { img_h };

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera_center = point3(0.0, 0.0, 0.0);

    // calculate the vectors across the horizontal and down the verical viewport edges
    let viewport_u = vec3(viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / img_width.into();
    let pixel_delta_v = viewport_v / img_height.into();

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut out = io::stdout().lock();

    writeln!(out, "P3\n{} {}\n255", img_width, img_height)?;

    for j in 0..img_height {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..img_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            pixel_color.write_to(&mut out)?;
        }
    }

    Ok(())
}
