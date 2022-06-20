mod color;
mod vec3;
mod ray;
mod sphere;
mod object;
mod plane;
mod camera;

use color::*;
use rand::prelude::Distribution;
use vec3::*;
use ray::*;
use object::*;
use camera::*;

use std::fs::File;
use std::io::{prelude::*, BufWriter};

fn div_by(x:i32) -> impl Fn(i32) -> f64 + 'static {
    move |y: i32| (y as f64 / x as f64)
}

fn ray_colour(r: Ray<f64>, environment: &Vec<Box<dyn Object>>) -> Color {
    // check if ray intersects an object in the environment
    // Note that we return the first intersection found. This assumes there are no overlapping objects
    let mut ts: Vec<(f64, &Box<dyn Object>)> =
        environment
            .iter()
            .flat_map(|obj| { 
                let t = obj.intersection(&r)?;
                Some(()).filter(|()| t >= 0.0)?;
                Some((t,obj))
            })
            .collect();

    ts.sort_by(|(x,_), (y,_)| f64::partial_cmp(&x,&y).expect("Couldn't sort f64 in ts"));

    for (t, obj) in ts {
        let intersection = r.at(t);
        let n = obj.normal(&intersection);
        return Color::from_unit(n);
    }

    let direction = unit::in_direction(r.direction);
    let t = (1.0 + direction.y)/2.0;

    gradient(WHITE, Color::new(0.5,0.7,1.0), t)
}

fn render<T: Write>(image_width: i32, samples_per_pixel: i32, camera: &Camera, environment: &Vec<Box<dyn Object>>, output: &mut T) -> std::io::Result<()> {
    let image_height = (image_width as f64 / camera.aspect_ratio) as i32;

    let mut gen = rand_pcg::Pcg64Mcg::new(0xcafef00dd15ea5e5);

    let rand_float_between_0_and_1 = rand::distributions::Uniform::new(0.0,1.0);

    output.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    
    for (j,v) in (0..image_height).rev().map(div_by(image_height - 1)).enumerate() {
        println!("Scanlines remaining {}", image_height as usize - j);
      
        for u in (0..image_width).map(div_by(image_width-1)) {   
            let mut pixel_colour = Color::new(0.0,0.0,0.0);

            for _ in 1..=samples_per_pixel {
                let v_jitter: f64 = rand_float_between_0_and_1.sample(&mut gen)/(image_height as f64);
                let h_jitter: f64 = rand_float_between_0_and_1.sample(&mut gen)/(image_width  as f64);

                let this_ray = camera.get_ray(u + h_jitter, v + v_jitter);
                
                pixel_colour += ray_colour(this_ray, &environment);
            }

            output.write(
                pixel_colour.scale(1.0/samples_per_pixel as f64).write_color().as_bytes()
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
    

    render(700, 16, &camera, &environment, &mut file_writer)?;

    Ok(())

    
}
