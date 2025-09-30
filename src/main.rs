use tikus::{
    camera::{Camera, render},
    color::color,
    hittable_list::HittableList,
    material::Material,
    math::{deg2rad, dvec3, point3},
    sphere::Sphere,
};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let material_ground = Material::new_lambertian(color(0.8, 0.8, 0.0));
    let material_center = Material::new_lambertian(color(0.1, 0.2, 0.5));
    let material_left = Material::new_dielectric(1.5);
    let material_bubble = Material::new_dielectric(1.0 / 1.5);
    let material_right = Material::new_metal(color(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere {
        center: point3(0., -100.5, -1.0),
        radius: 100.0,
        mat: material_ground,
    });
    world.add(Sphere {
        center: point3(0., 0., -1.2),
        radius: 0.5,
        mat: material_center,
    });
    // world.add(Sphere {
    //     center: point3(0.0, -0.3, -0.5),
    //     radius: 0.3,
    //     mat: Material::new_lambertian(color(0.5, 0., 0.5)),
    // });
    world.add(Sphere {
        center: point3(-1.0, 0., -1.2),
        radius: 0.5,
        mat: material_left,
    });
    world.add(Sphere {
        center: point3(-1.0, 0., -1.2),
        radius: 0.4,
        mat: material_bubble,
    });
    world.add(Sphere {
        center: point3(1.0, 0., -1.2),
        radius: 0.5,
        mat: material_right,
    });

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        img_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: deg2rad(20.0),
        lookfrom: point3(-2., 2., 1.),
        lookat: point3(0., 0., -1.),
        vup: dvec3(0., 1., 0.),
        defocus_angle: deg2rad(10.0),
        focus_dist: 3.4,
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
