use crate::ray::*;
use crate::vec3::*;

#[derive(Debug)]
pub enum Material {
    Mirror,
    Diffuse
}

pub trait Object {
    fn intersection(&self, ray: &Ray<f64>) -> Option<f64>;

    /// Unit vector normal on object at intersection
    /// This vector points outward on a closed object
    fn normal(&self, intersection: &Vec3<f64>) -> Vec3<f64>;
    
    fn material(&self) -> &Material;
}