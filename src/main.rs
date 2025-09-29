use tikus::{
    camera::{Camera, render},
    color::color,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
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

    let material_ground = Lambertian {
        albedo: color(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: color(0.1, 0.2, 0.5),
    };
    let material_left = Metal {
        albedo: color(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Metal {
        albedo: color(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Sphere {
        center: point3(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: material_ground,
    });
    world.add(Sphere {
        center: point3(0.0, 0.0, -1.2),
        radius: 0.5,
        mat: material_center,
    });
    world.add(Sphere {
        center: point3(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_left,
    });
    world.add(Sphere {
        center: point3(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_right,
    });

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        img_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    let mut args = std::env::args();
    let program_name = args.next().expect("program name should be available");
    let filename = args.next().unwrap_or_else(|| {
        println!("no filename given, defaulting to `image.ppm`");
        "image.ppm".to_string()
    });

    println!("{} v0.0.0", program_name);
    println!("Config.");
    println!("aspect_ratio      : {}", camera.aspect_ratio);
    println!("img_width         : {}", camera.img_width);
    println!("samples_per_pixel : {}", camera.samples_per_pixel);
    println!("max_depth         : {}", camera.max_depth);
    println!("\nOutput -> {}", filename);

    render(filename, &camera, &world)?;
    Ok(())
}
