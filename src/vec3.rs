use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}



pub mod unit {
    use super::*;
    pub const X: Vec3<f64> = Vec3::new(1.0,0.0,0.0);
    pub const Y: Vec3<f64> = Vec3::new(0.0,1.0,0.0);
    pub const Z: Vec3<f64> = Vec3::new(0.0,0.0,1.0);

    pub fn in_direction(v: Vec3<f64>) -> Vec3<f64> {
        v.inv_scale(v.length())
    }
}


impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T> Vec3<T>
where T: Mul<Output = T> + Add<Output = T> + Copy
{
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    pub fn scale(&self, other: T) -> Self {
        let new_x = self.x * other;
        let new_y = self.y * other;
        let new_z = self.z * other;
        
        Vec3::new(new_x, new_y, new_z)
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }
}


impl<T> Vec3<T>
where T: Div<Output = T> + Copy
{
    pub fn inv_scale(&self, other: T) -> Self {
        let new_x = self.x / other;
        let new_y = self.y / other;
        let new_z = self.z / other;
        
        Vec3::new(new_x, new_y, new_z)
    }
}



impl Vec3<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}


impl<T> Neg for Vec3<T>
where T: Neg<Output = T> {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        let new_x = -self.x;
        let new_y = -self.y;
        let new_z = -self.z;
        
        Vec3::new(new_x, new_y, new_z)
    }
}

impl<T> Add for Vec3<T>
where T: Add<Output = T> {
    type Output = Self;
    
    fn add(self, other: Vec3<T>) -> Self::Output {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        let new_z = self.z + other.z;
        
        Vec3::new(new_x, new_y, new_z)
    }
}


impl<T> Sub for Vec3<T>
where T: Sub<Output = T> {
    type Output = Self;
    
    fn sub(self, other: Vec3<T>) -> Self::Output {
        let new_x = self.x - other.x;
        let new_y = self.y - other.y;
        let new_z = self.z - other.z;

        Vec3::new(new_x, new_y, new_z)
    }
}

impl<T> MulAssign<T> for Vec3<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}


impl<T> Div<T> for Vec3<T>
where T: Div<Output = T> + Copy 
{
    type Output = Self;
    
    fn div(self, rhs: T) -> Self {
        let new_x = self.x / rhs;
        let new_y = self.y / rhs;
        let new_z = self.z / rhs;
        
        Vec3::new(new_x, new_y, new_z)
    }
}


impl<T> DivAssign<T> for Vec3<T>
    where T: DivAssign<T> + Copy
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}


impl<T> AddAssign for Vec3<T>
    where T: AddAssign
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


impl<T> SubAssign for Vec3<T>
    where T: SubAssign
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

