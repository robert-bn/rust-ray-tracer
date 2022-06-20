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



const MAX_BOUNCES: u64 = 50;
const IMAGE_WIDTH: u64 = 400;
const SAMPLES_PER_PIXEL: u64 = 100;


fn ray_colour<R: Rng>(r: Ray<f64>, environment: &Vec<Box<dyn Object>>, depth: u64, rng: &mut R) -> Color {
    // check if ray intersects an object in the environment
    // Note that we return the first intersection found. This assumes there are no overlapping objects

    // maximum bounces exceeded
    if depth == 0 { return BLACK }

    let hit: Option<(f64, &Box<dyn Object>)> = environment
        .iter()
        .flat_map(|obj| obj.intersection(&r).and_then(|t| (t >= 0.001 /* Shadow acne */).then(|| (t,obj))))
        .min_by(|(x,_), (y,_)| f64::partial_cmp(&x,&y).expect("Couldn't sort f64 in hits"));

    match hit {
        Some((t, obj)) => {
            let intersection = r.at(t);
            let n = obj.normal(&intersection);
            let scatter_direction = n + unit::random(rng);
            let new_ray = Ray { origin: intersection, direction: scatter_direction };
            ray_colour(new_ray, environment, depth - 1, rng).on_vec(|v| v / 2.0)
        },
        None => {
            let direction = unit::in_direction(r.direction);
            let t = (1.0 + direction.y)/2.0;
            gradient(WHITE, Color::new(0.5,0.7,1.0), t)
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

    let samples_per_pixel_f64 = samples_per_pixel as f64;

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
                pixel_colour.on_vec(|v| v / samples_per_pixel_f64).write_color().as_bytes()
            )?;
        }
    }

    Ok(())
}


fn main() -> std::io::Result<()> {
    use sphere::*;
    use plane::*;
    use camera::*;

    let camera = default_camera();

    let file = File::create("image.ppm")?;

    let mut file_writer = BufWriter::new(file);
    
    let environment: Vec<Box<dyn Object>> =
    vec![ Box::new(Sphere { radius: 0.5, centre: Vec3::new(0.0, 0.0, -1.0) })
        , Box::new(Sphere { radius: 100.0, centre: Vec3::new( 0.0, -100.5, -1.0) })
        // , Box::new(Plane::new(unit::Y, -0.1))
        ];
    

    render(IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_BOUNCES, &camera, &environment, &mut file_writer)?;

    Ok(())

    
}
