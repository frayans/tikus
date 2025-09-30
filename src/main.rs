use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use tikus::{
    camera::{Camera, render},
    color::color,
    hittable_list::HittableList,
    material::Material,
    math::point3,
    sphere::Sphere,
};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let r = FRAC_PI_4.cos();

    let material_left = Material::new_lambertian(color(0., 0., 1.));
    let material_right = Material::new_lambertian(color(1., 0., 0.));

    world.add(Sphere {
        center: point3(-r, 0., -1.),
        radius: r,
        mat: material_left,
    });
    world.add(Sphere {
        center: point3(r, 0., -1.),
        radius: r,
        mat: material_right,
    });

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        img_width: 640,
        samples_per_pixel: 128,
        max_depth: 50,
        vfov: FRAC_PI_2,
    };

    let mut args = std::env::args();
    let program_name = args.next().expect("program name should be available");
    let filename = args.next().unwrap_or_else(|| {
        let default_fn = format!("{}w{}spp.ppm", camera.img_width, camera.samples_per_pixel);
        println!("no filename given, defaulting to `{}`", default_fn);
        default_fn
    });

    println!("{} v0.0.0", program_name);
    println!("Config.");
    println!("{:#?}", camera);
    println!("\nOutput -> {}", filename);

    render(filename, &camera, &world)?;
    Ok(())
}
