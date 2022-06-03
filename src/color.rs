use std::ops::Add;

use crate::vec3::Vec3;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color(Vec3<f64>);

impl Color {
    pub const fn new(r: f64, b: f64, g: f64) -> Self {
        Color(Vec3::new(r,g,b))
    }

    pub fn from_unit(n: Vec3<f64>) -> Self {
        // Create a colour from a unit vector, where x,y,z in range [-1,1] are mapped onto [0,1] for r,g,b
        Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0).scale(0.5)
    }
    
    pub fn write_color(&self) {
        let ir = (self.0.x * 255.0).round() as u8;
        let ig = (self.0.z * 255.0).round() as u8;
        let ib = (self.0.y * 255.0).round() as u8;
        
    println!("{} {} {}", ir, ig, ib)
    }

    pub fn scale(&self, scale_factor: f64) -> Self {
        Color(self.0.scale(scale_factor))
    }
}

pub fn gradient(from: Color, to: Color, t: f64) -> Color {
    // linear blend between two colours paramizted by t in range [0.0,1.0]
    from.scale(1.0 - t) + to.scale(t)
}

impl Add for Color {
    type Output = Self;
    
    fn add(self, other: Color) -> Self::Output {
        Color(self.0 + other.0)
    }
}

pub const WHITE: Color = Color::new(1.0,1.0,1.0);
pub const BLACK: Color = Color::new(0.0,0.0,0.0);
pub const RED:   Color = Color::new(1.0,0.0,0.0);
pub const GREEN: Color = Color::new(0.0,1.0,0.0);
pub const BLUE:  Color = Color::new(0.0,0.0,1.0);

