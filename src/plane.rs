use crate::vec3::*;
use crate::object::*;
use crate::ray::*;

#[derive(Debug)]
pub struct Plane {
    unit_normal: Vec3<f64>,
    origin_distance: f64,
    material: Material
}

impl Plane {
    pub fn new(normal: Vec3<f64>, point_in_plane: Vec3<f64>, material: Material) -> Self {
        let unit_normal = unit::in_direction(normal);
        let origin_distance = point_in_plane.dot(&unit_normal);
        Plane { unit_normal, origin_distance, material }
    }
}

impl Object for Plane {
    fn intersection(&self, ray: &Ray<f64>) -> Option<f64> {
        let t = (self.origin_distance - Vec3::dot(&self.unit_normal, &ray.origin))
                / Vec3::dot(&self.unit_normal, &ray.direction);
        
        Some(t).filter(|&t|  t.is_sign_positive() && t.is_normal())
    }

    fn normal(&self, _intersection: &Vec3<f64>) -> Vec3<f64> {
        self.unit_normal
    }

    fn material(&self) -> &Material {
        &self.material
    }
}