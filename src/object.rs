use std::ops::Deref;

use rand::Rng;

use crate::ray::*;
use crate::vec3::*;
use crate::color::*;

#[derive(Debug,Clone,Copy)]
pub enum Material {
    Diffuse { absorb: Color },
    Reflective { absorb: Color, roughness: f64 },
    Dialectric { absorb: Color, reflection_prob: f64, roughness: f64 }
}

impl Material {
    pub fn absorb(self) -> Color {
        match self {
            Material::Diffuse { absorb } => absorb,
            Material::Reflective { absorb, roughness: _ } => absorb,
            Material::Dialectric { absorb, reflection_prob: _, roughness: _ } => absorb,
        }
    }
}

pub fn interact<R: Rng>(obj: &Box<dyn Object>, incoming_ray: Ray<f64>, t:f64, rng: &mut R) -> Ray<f64> {
    let intersection = incoming_ray.at(t);
    let unit_normal = obj.normal(&intersection);
    let outgoing_direction: Vec3<f64> = match obj.material() {
        Material::Diffuse { absorb: _ } => {
            /* unit_normal and the random unit vector could be in the opposite direction,
                in which case scatter_direction will be zero. This can lead to floating
                point errors, so in this case we set the scatter direction to the normal
            */
            let proposed = unit_normal + unit::random(rng);
            if proposed.near_zero() { unit_normal } else { proposed }
        },
        Material::Reflective { absorb: _, roughness } => {
            let incoming_direction = unit::in_direction(incoming_ray.direction);
            let reflected_direction = incoming_direction - (unit_normal * 2.0 * incoming_direction.dot(&unit_normal));
            let fuzz = random_in_unit_sphere(rng) * (*roughness);
            let proposed = reflected_direction + fuzz;
            if proposed.dot(&unit_normal) < 0.0 { 
                return Ray { origin: intersection, direction: unit_normal, color: obj.material().absorb() }
             } else { proposed }
        },
        Material::Dialectric { absorb: _, reflection_prob: _, roughness: _ } => todo!(),
    };

    Ray { origin: intersection, direction: outgoing_direction, color: incoming_ray.color * obj.material().absorb() }
}


pub trait Object {
    fn intersection(&self, ray: &Ray<f64>) -> Option<f64>;

    /// Unit vector normal on object at intersection
    /// This vector points outward on a closed object
    fn normal(&self, intersection: &Vec3<f64>) -> Vec3<f64>;
    
    fn material(&self) -> &Material;
}