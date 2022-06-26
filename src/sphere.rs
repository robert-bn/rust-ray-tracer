use crate::vec3::*;
use crate::ray::*;
use crate::object::*;
use super::SHADOW_ACNE_TOLERANCE;

#[derive(Debug)]
pub struct Sphere {
    pub radius: f64,
    pub centre: Vec3<f64>,
    pub material: Material
}

impl Object for Sphere {
    fn intersection(&self, ray: &Ray<f64>) -> Option<f64> {
        let v_origin_centre = ray.origin - self.centre;
    
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&v_origin_centre);  // h = 2b in quadratic equatino
        let c = v_origin_centre.length_squared() - self.radius*self.radius;
    
        let descriminant = h*h - a*c;
    
        if descriminant < 0.0 {
            return None;
        }
    
        let part2 = f64::sqrt(descriminant)/a;
        let part1 = -h/a;
    
        let t_0 = part1 - part2;
        let t_1 = part1 + part2;
    
        // Since the ray comes from the camera, t = 0 is the origin, 
        // t closer to camera should always be the (absolute value) smaller one?
        // If t is negative, then the intersection is behind the camera, so we don't want to
        // show it at all really.
    
        // We could have positive t1, negative t0 etc, if the camera is INSIDE the sphere
        // In this case we want the positive intersection which should be IN FRONT of the
        // camera, and the negative one behind it.
    
        match (t_0 > SHADOW_ACNE_TOLERANCE, t_1 > SHADOW_ACNE_TOLERANCE) {
            (true, _)      => Some(t_0),
            (false, true)  => Some(t_1),
            (false, false) => None,
        }
    }

    fn normal(&self, intersection: &Vec3<f64>) -> Vec3<f64> {
        let sphere_center_to_intersection = *intersection - self.centre;
        unit::in_direction(sphere_center_to_intersection)
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

