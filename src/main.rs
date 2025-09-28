use tikus::{
    camera::{Camera, render},
    hittable_list::HittableList,
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

    world.add(Sphere {
        center: point3(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: point3(0.0, -100.5, -1.0),
        radius: 100.0,
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
