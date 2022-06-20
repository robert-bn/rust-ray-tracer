use rand::Rng;

use crate::ray::*;
use crate::vec3::*;
use crate::color::*;

#[derive(Debug)]
pub struct Material {
    pub reflection_prob: f64,
    // pub scatter_prob: f64,
    pub absorb: Color
}

pub enum Interaction {
    Scatter,
    Reflect
}

impl Material {
    pub fn interact<R: Rng>(&self, rng: &mut R) -> Interaction {
        if self.reflection_prob > 0.0 && rng.gen::<f64>() < self.reflection_prob {
            Interaction::Reflect
        } else {
            Interaction::Scatter
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