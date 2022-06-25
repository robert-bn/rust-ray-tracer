use rand::Rng;

use crate::ray::*;
use crate::vec3::*;
use crate::color::*;

const AIR_REFRACTIVE_INDEX: f64 = 1.0;

#[derive(Debug,Clone,Copy)]
pub enum Material {
    Diffuse { absorb: Color },
    Reflective { absorb: Color, roughness: f64 },
    Dialectric { absorb: Color, roughness: f64, refractive_index: f64 }
}

impl Material {
    pub fn absorb(self) -> Color {
        match self {
            Material::Diffuse { absorb } => absorb,
            Material::Reflective { absorb, roughness: _ } => absorb,
            Material::Dialectric { absorb, roughness: _, refractive_index: _ } => absorb,
        }
    }
}


pub fn interact<R: Rng>(obj: &Box<dyn Object>, incoming_ray: Ray<f64>, t:f64, rng: &mut R) -> Ray<f64> {
    let intersection = incoming_ray.at(t);
    let unit_normal = obj.normal(&intersection);
    let reflect = |absorb: Color, roughness: f64, rng: &mut R| -> Ray<f64>  {
        let incoming_direction = unit::in_direction(incoming_ray.direction);
        let reflected_direction = incoming_direction - (unit_normal * 2.0 * incoming_direction.dot(&unit_normal));
        let fuzz = random_in_unit_sphere(rng) * roughness;
        let proposed = reflected_direction + fuzz;
        if proposed.dot(&unit_normal) < 0.0 { 
            Ray { origin: intersection, direction: unit_normal, color: absorb }
        } else { 
            Ray { origin: intersection, direction: proposed, color: incoming_ray.color * obj.material().absorb() }
        }
    };
    match *obj.material() {
        Material::Diffuse { absorb } => {
            /* unit_normal and the random unit vector could be in the opposite direction,
                in which case scatter_direction will be zero. This can lead to floating
                point errors, so in this case we set the scatter direction to the normal
            */
            let proposed = unit_normal + unit::random(rng);
            let outgoing_direction = if proposed.near_zero() { unit_normal } else { proposed };
            Ray { origin: intersection, direction: outgoing_direction, color: incoming_ray.color * absorb }
        },
        Material::Reflective { absorb, roughness } => reflect(absorb, roughness, rng),
        Material::Dialectric { absorb, roughness, refractive_index } => {
            let incoming_dot_normal = incoming_ray.direction.dot(&unit_normal);
            let inside_material = incoming_dot_normal > 0.0;
            let unit_perp = unit_normal * f64::signum(incoming_dot_normal);
            let incoming_dot_perp = f64::abs(incoming_dot_normal);

            let unit_parallel = unit::in_direction(incoming_ray.direction - unit_perp * incoming_dot_perp);

            let (n1,n2) =
                if inside_material { (refractive_index, AIR_REFRACTIVE_INDEX) }  // ray entering material from outside
                else               { (AIR_REFRACTIVE_INDEX, refractive_index) }; // ray exiting material

            let cos_t1 = incoming_dot_normal / incoming_ray.direction.length();

            
            let sin_t1_squared = 1.0 - cos_t1.powi(2);
            let sin_t2_squared = (n1/n2).powi(2) * sin_t1_squared;

            if sin_t2_squared > 1.0 {
                // Exceeded angle of total internal reflection, must reflect ray
                reflect(absorb, roughness, rng)
            } else {
                let sin_t2 = f64::sqrt(sin_t2_squared);
                let cos_t2 = f64::sqrt(1.0 - sin_t2_squared);
                
                
                let outgoing_direction = unit_perp * cos_t2 + unit_parallel * sin_t2;
                Ray { origin: intersection, direction: outgoing_direction, color: incoming_ray.color }
            }
        }
    }
}


pub trait Object {
    fn intersection(&self, ray: &Ray<f64>) -> Option<f64>;

    /// Unit vector normal on object at intersection
    /// This vector points outward on a closed object
    fn normal(&self, intersection: &Vec3<f64>) -> Vec3<f64>;
    
    fn material(&self) -> &Material;
}