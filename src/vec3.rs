#![allow(dead_code)]
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

// Define some type aliases
pub type Point3 = Vec3;
pub type Color = Vec3;

// Make a convenient macro
#[macro_export]
macro_rules! vector3 {
    ($a: expr, $b: expr, $c: expr) => {
        crate::vec3::Vec3::new($a as f64, $b as f64, $c as f64)
    }
}

#[macro_export]
macro_rules! point3 {
    ($a: expr, $b: expr, $c: expr) => {
        crate::vec3::Point3::new($a as f64, $b as f64, $c as f64)
    }
}

#[macro_export]
macro_rules! color_rgb {
    ($a: expr, $b: expr, $c: expr) => {
        crate::vec3::Color::new($a as f64, $b as f64, $c as f64)
    }
}

// Other functions that aren't methods
pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.dot(b)
}

// Some simple implicated functions that don't rely on traits
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    // Simple xyz stuff
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    // The magnitude of the vector
    pub fn length(&self) -> f64 {
        let sqr_sum = self[0].powi(2) + self[1].powi(2) + self[2].powi(2);
        sqr_sum.sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        let temp = self * other;
        temp[0] + temp[1] + temp[2]
    }

    pub fn cross(self, other: Self) -> Self {
        let i = self[1] * other[2] - self[2] * other[1];
        let j = self[2] * other[0] - self[0] * other[2];
        let k = self[0] * other[1] - self[1] * other[0];
        vector3!(i, j, k)
    }

    pub fn unit(self) -> Self {
        let l = self.length();
        self / l
    }
}

// Implement traits that make working with this class easier
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

// Addition traits
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let e = [self[0] + other[0], self[1] + other[1], self[2] + other[2]];
        Self { e }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        for i in 0..3 {
            self[i] += other[i];
        }
    }
}

// Subtraction traits
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

// Multiplication traits
impl Mul for Vec3 {
    // Note that this is not a true dot or cross product
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    // This is really just for commutivity
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self[i] *= rhs;
        }
    }
}

// Division trait
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f64)
    }
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), vector3!(1.0, 2.0, 3.0));
    }

    #[test]
    fn add() {
        assert_eq!(vector3!(1, 2, 3) + vector3!(3, 2, 1), vector3!(4, 4, 4));
    }

    #[test]
    fn add_assign() {
        let mut a = vector3!(1, 2, 3);
        a += vector3!(3, 2, 1);
        assert_eq!(a, vector3!(4, 4, 4));
    }

    #[test]
    fn sub() {
        let a = vector3!(1, 2, 3);
        let b = vector3!(3, 2, 1);
        assert_eq!(a - b, vector3!(-2, 0, 2));
    }

    #[test]
    fn mul() {
        let a = vector3!(1, 2, 3);
        let b = a.clone();
        let c = a.clone() * b.clone();
        assert_eq!(a * 2.0, vector3!(2, 4, 6));
        assert_eq!(2.0 * b, vector3!(2, 4, 6));
        assert_eq!(c, vector3!(1, 4, 9));
    }

    #[test]
    fn length() {
        let a = vector3!(0, 3, 4);
        assert_eq!(a.length(), 5.0);
    }

    #[test]
    fn dot_product() {
        let a = vector3!(1, 2, 3);
        let b = vector3!(4, 6, 2);
        assert_eq!(a.dot(b), 22.0);
    }
}