mod color;
mod vec3;
mod ray;
mod sphere;
mod object;
mod plane;

use color::*;
use vec3::*;
use ray::*;
use object::*;

fn div_by(x:i32) -> impl Fn(i32) -> f64 + 'static {
    move |y: i32| (y as f64 / x as f64)
}

fn ray_colour(r: Ray<f64>, environment: &Vec<Box<dyn Object>>) -> Color {
    // check if ray intersects an object in the environment
    // Note that we return the first intersection found. This assumes there are no overlapping objects
    for n in
        environment.iter()
            .map(|obj| obj.intersection(&r)
            .map(|intersection| obj.normal(&intersection)))
            .flatten() {
        return Color::from_unit(n);
    }

    let direction = unit::in_direction(r.direction);
    let t = (1.0 + direction.y)/2.0;

    gradient(WHITE, Color::new(0.5,0.7,1.0), t)
}


fn main() {
    use sphere::*;
    use plane::*;

    let aspect_ratio = 16.0/9.0;
    let image_width  = 700;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let focal_length = 1.0;
    
    let origin: Vec3<f64> = Vec3::new(0.0, 0.0, 0.0);
    
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    
    
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical   = Vec3::new(0.0,viewport_height,0.0);
    
    let bottom_left = origin - horizontal.scale(0.5) - vertical.scale(0.5) - Vec3::new(0.0,0.0,focal_length);

    let environment: Vec<Box<dyn Object>> =
        vec![ Box::new(Sphere { radius: 0.3, centre: Vec3::new(-0.4, 0.0, -1.0) })
            , Box::new(Sphere { radius: 0.3, centre: Vec3::new( 0.4, 0.0, -1.0) })
            , Box::new(Plane::new(unit::Y, -1.0))
            ];

    println!("P3\n{} {}\n255\n", image_width, image_height);
    
    for (j,v) in (0..image_height).rev().map(div_by(image_height - 1)).enumerate() {
        eprintln!("Scanlines remaining {}", image_height as usize - j);
      
        
        for u in (0..image_width).map(div_by(image_width-1)) {   

            let direction
                = bottom_left
                + horizontal.scale(u)
                + vertical.scale(v)
                - origin;
            
            let this_ray = Ray { origin, direction };

            ray_colour(this_ray, &environment).write_color();
        }
    }
}
