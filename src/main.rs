mod color;
mod vec3;
mod ray;
mod sphere;
mod object;
mod plane;
mod camera;

use color::*;
use rand::{Rng, prelude::*};
use vec3::*;
use ray::*;
use object::*;
use camera::*;

use std::fs::File;
use std::io::{prelude::*, BufWriter};



const MAX_BOUNCES: u64 = 8;
const IMAGE_WIDTH: u64 = 800;
const SAMPLES_PER_PIXEL: u64 = 100;
const SHADOW_ACNE_TOLERANCE: f64 = 0.0001;
const KILLING_WEIGHT: f64 = 1.0 / 255.0;


fn ray_colour<R: Rng>(incoming_ray: Ray<f64>, environment: &Vec<Box<dyn Object>>, depth: u64, rng: &mut R) -> Color {
    // weight too low, kill
    if incoming_ray.color.weight() < KILLING_WEIGHT { return incoming_ray.color }

    // maximum bounces exceeded
    if depth == 0 { return BLACK }

    let hit: Option<(f64, &Box<dyn Object>)> = environment
        .iter()
        .flat_map(|obj| obj.intersection(&incoming_ray).and_then(|t| (t >= SHADOW_ACNE_TOLERANCE).then(|| (t,obj))))
        .min_by(|(x,_), (y,_)| f64::partial_cmp(&x,&y).expect("Couldn't sort f64 in hits"));

    match hit {
        Some((t, obj)) => {
            let outgoing_ray = object::interact(obj, incoming_ray, t, rng);
            ray_colour(outgoing_ray, environment, depth - 1, rng)
        },
        None => {
            let direction = unit::in_direction(incoming_ray.direction);
            let t = (1.0 + direction.y)/2.0;
            incoming_ray.color * gradient(WHITE, Color::new(0.5,0.7,1.0), t)
        }
    }
}

fn render<T: Write>(
    image_width: u64,
    samples_per_pixel: u64,
    max_bounces: u64,
    camera: &Camera,
    environment: &Vec<Box<dyn Object>>,
    output: &mut T
) -> std::io::Result<()> {
    fn div_by(x:u64) -> impl Fn(u64) -> f64 + 'static {
        move |y: u64| (y as f64 / x as f64)
    }

    let image_height = (image_width as f64 / camera.aspect_ratio) as u64;

    let mut rng = rand_pcg::Pcg64Mcg::new(0xcafef00dd15ea5e5);

    let v_jitter_dist = rand::distributions::Uniform::new(-1.0/image_height as f64,1.0/image_height as f64);
    let h_jitter_dist = rand::distributions::Uniform::new(-1.0/image_width as f64,1.0/image_width as f64);

    output.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    
    for (j,v) in (0..image_height).rev().map(div_by(image_height - 1)).enumerate() {
        println!("Scanlines remaining {}", image_height as usize - j);
      
        for u in (0..image_width).map(div_by(image_width-1)) {   
            let mut pixel_colour = BLACK;

            for _ in 1..=samples_per_pixel {
                let v_jitter: f64 = v_jitter_dist.sample(&mut rng);
                let h_jitter: f64 = h_jitter_dist.sample(&mut rng);

                let this_ray = camera.get_ray(u + h_jitter, v + v_jitter);
                
                pixel_colour += ray_colour(this_ray, &environment, max_bounces, &mut rng);
            }

            output.write(
                pixel_colour.on_vec(|v| v / samples_per_pixel as f64).write_color().as_bytes()
            )?;
        }
    }

    Ok(())
}


fn main() -> std::io::Result<()> {
    use sphere::*;
    // use plane::*;
    use camera::*;

    let camera = default_camera();

    let file = File::create("image.ppm")?;

    let mut file_writer = BufWriter::new(file);

    let material_ground = Material::Diffuse { absorb: Color::new(0.8, 0.8, 0.0) };
    let material_center = Material::Diffuse { absorb: Color::new(0.7, 0.3, 0.3) };
    let material_left   = Material::Reflective { absorb: Color::new(0.8, 0.8, 0.8), roughness:0.3 };
    let material_right  = Material::Reflective { absorb: Color::new(0.8, 0.6, 0.2), roughness:1.0 };
    
    let environment: Vec<Box<dyn Object>> =
    vec![ Box::new(Sphere { radius: 0.5,   centre: Vec3::new(-0.0,   0.0,  -1.0), material: material_center })
        , Box::new(Sphere { radius: 0.5,   centre: Vec3::new(-1.0,   0.0,  -1.0), material: material_left   })
        , Box::new(Sphere { radius: 0.5,   centre: Vec3::new( 1.0,   0.0,  -1.0), material: material_right  })
        , Box::new(Sphere { radius: 100.0, centre: Vec3::new( 0.0,-100.5,  -1.0), material: material_ground })
        ];    

    render(IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_BOUNCES, &camera, &environment, &mut file_writer)?;

    Ok(())

    
}
