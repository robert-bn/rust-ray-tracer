use std::ops::{Add, AddAssign};

use crate::vec3::Vec3;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color(Vec3<f64>);

impl Color {
    pub const fn r(&self) -> f64 { self.0.x }
    pub const fn g(&self) -> f64 { self.0.y }
    pub const fn b(&self) -> f64 { self.0.z }

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r,g,b))
    }

    pub fn from_absorbtion(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(1.0-r,1.0-g,1.0-b))
    }

    pub fn from_unit(n: Vec3<f64>) -> Self {
        // Create a colour from a unit vector, where x,y,z in range [-1,1] are mapped onto [0,1] for r,g,b
        Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0).on_vec(|v| v / 2.0)
    }
    
    pub fn write_color(&self) -> String {
        fn clamp(x: f64) -> f64 {
            if x > 1.0 { 1.0 } else { x }
        }

        let ir = (clamp(f64::sqrt(self.r())) * 255.0).round() as u8;
        let ig = (clamp(f64::sqrt(self.g())) * 255.0).round() as u8;
        let ib = (clamp(f64::sqrt(self.b())) * 255.0).round() as u8;

        format!("{} {} {}\n", ir, ig, ib)
    }

    pub fn on_vec<F>(self, f: F) -> Self
        where F: FnOnce(Vec3<f64>) -> Vec3<f64>
    {
        Color(f(self.0))
    }

    pub fn absorb(self, other: &Color) -> Self {
        let new_r = self.r() * other.r();
        let new_g = self.g() * other.g();
        let new_b = self.b() * other.b();

        Color::new(new_r, new_g, new_b)
    }
}

pub fn gradient(from: Color, to: Color, t: f64) -> Color {
    // linear blend between two colours paramizted by t in range [0.0,1.0]
    from.on_vec(|v| v * (1.0 - t)) + to.on_vec(|v| v * t)
}

impl Add for Color {
    type Output = Self;
    
    fn add(self, other: Color) -> Self::Output {
        Color(self.0 + other.0)
    }
}


impl AddAssign for Color
{
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}


pub const WHITE: Color = Color::new(1.0,1.0,1.0);
pub const BLACK: Color = Color::new(0.0,0.0,0.0);
pub const RED:   Color = Color::new(1.0,0.0,0.0);
pub const GREEN: Color = Color::new(0.0,1.0,0.0);
pub const BLUE:  Color = Color::new(0.0,0.0,1.0);
pub const GREY_50: Color = Color::new(0.5,0.5,0.5);

