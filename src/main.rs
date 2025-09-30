use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use tikus::{
    camera::{Camera, render},
    color::color,
    hittable_list::HittableList,
    material::Material,
    math::{deg2rad, dvec3, point3, random, random_double, random_range},
    sphere::Sphere,
};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let ground_mat = Material::new_lambertian(color(0.5, 0.5, 0.5));
    world.add(Sphere {
        center: point3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: ground_mat,
    });

    for a in -11..11 {
        for b in -11..11 {
            let mut rng = Xoshiro256Plus::seed_from_u64((a * a) as u64 + (b * b) as u64 ^ 1234);
            let choose_mat = random_double(&mut rng);
            let center = point3(
                a as f64 * random_double(&mut rng),
                0.2,
                b as f64 + 0.9 * random_double(&mut rng),
            );

            if (center - point3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random(&mut rng) * random(&mut rng);
                    let sphere_mat = Material::new_lambertian(albedo);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_mat,
                    });
                } else if choose_mat < 0.95 {
                    let albedo = random_range(&mut rng, 0.5..1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_mat = Material::new_metal(albedo, fuzz);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_mat,
                    });
                } else {
                    let sphere_mat = Material::new_dielectric(1.5);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_mat,
                    });
                }
            }
        }
    }

    let mat = Material::new_dielectric(1.5);
    world.add(Sphere {
        center: point3(0., 1., 0.),
        radius: 1.,
        mat,
    });

    let mat = Material::new_lambertian(color(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: point3(-4., 1., 0.),
        radius: 1.,
        mat,
    });

    let mat = Material::new_metal(color(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere {
        center: point3(4., 1., 0.),
        radius: 1.,
        mat,
    });

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        img_width: 1280,
        samples_per_pixel: 16,
        max_depth: 50,
        vfov: deg2rad(20.0),
        lookfrom: point3(12., 2., 3.),
        lookat: point3(0., 0., 0.),
        vup: dvec3(0., 1., 0.),
        defocus_angle: deg2rad(0.6),
        focus_dist: 10.,
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
