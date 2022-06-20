use std::ops::{Mul, Add};

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>
}

impl<T> Ray<T> {

    // linear interpolation along a line defined by origin and direction
    pub fn at(&self, t: T) -> Vec3<T>
    where T : Add<Output = T>
            + Mul<Output = T> 
            + Copy
    {
        self.origin + (self.direction * t)
    }
}