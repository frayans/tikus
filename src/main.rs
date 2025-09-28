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

    world.add(Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(point3(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        img_width: 400,
    };

    render(&camera, &world)?;
    Ok(())
}
