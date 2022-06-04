use crate::vec3::*;
use crate::object::*;
use crate::ray::*;

pub struct Plane {
    unit_normal: Vec3<f64>,
    pub origin_distance: f64
}

impl Plane {
    pub fn new(normal: Vec3<f64>, origin_distance: f64) -> Self {
        Plane { unit_normal: unit::in_direction(normal), origin_distance }
    }
}

impl Object for Plane {
    fn intersection(&self, ray: &Ray<f64>) -> Option<Vec3<f64>> {
        let t = (self.origin_distance - Vec3::dot(&self.unit_normal, &ray.origin))
                / Vec3::dot(&self.unit_normal, &ray.direction);
            
        if t <= 0.0 || !t.is_normal() {
            None
        } else {
            Some(ray.at(t))
        }
    }

    fn normal(&self, _intersection: &Vec3<f64>) -> Vec3<f64> {
        self.unit_normal
    }
}