use crate::color;
use crate::vec3::*;
use crate::ray::*;


pub struct Camera {
    origin: Vec3<f64>,
    bottom_left: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
            let direction
                = self.bottom_left
                + (self.horizontal * u)
                + (self.vertical * v)
                - self.origin;
        
        Ray { origin:self.origin, direction, color: color::WHITE }
    }
}


const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;
const DEFAULT_VIEWPORT_HEIGHT: f64 = 2.0;
const DEFAULT_VIEWPORT_WIDTH: f64 = DEFAULT_ASPECT_RATIO * DEFAULT_VIEWPORT_HEIGHT;

pub fn default_camera() -> Camera {
    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(DEFAULT_VIEWPORT_WIDTH,0.0,0.0);
    let vertical = Vec3::new(0.0,DEFAULT_VIEWPORT_HEIGHT,0.0);
    let bottom_left = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0,0.0,DEFAULT_FOCAL_LENGTH);
    
    Camera { origin, horizontal, vertical, bottom_left, aspect_ratio: DEFAULT_ASPECT_RATIO }
}

