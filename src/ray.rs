#![allow(dead_code)]
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

/// Use two lists to designate a shorthand Ray.
#[macro_export]
macro_rules! m_ray {
    () => {
        // Generate a default for testing purposes
        Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))
    };
    ($orig: expr, $dir: expr) => {
        // Both of these must be valid arrays
        Ray::new(Point3::new($orig[0] as f64, $orig[1] as f64, $orig[2] as f64), 
            Vec3::new($dir[0] as f64, $dir[1] as f64, $dir[2] as f64))
    }
}

impl Ray {
    /// Create a new instance of a Ray. Pass in a Point3 and a Vec3 in A + tB format. A is the starting point and 
    /// B is the direction. Note that a Point3 is the same thing as a Vec3 data type-wise.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
        }
    }

    pub fn from(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    /// This gets a point on the ray given a time t. This does not consume the Ray and still allows usage. However,
    /// it may be performance hindering if a Ray is made and only used once for evaluation. For immediate consuming after
    /// evaluation, use the evaluate method.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + t * self.dir.clone()
    }

    /// This function evaluates a point on the Ray and consumes it, yielding a Point3.
    pub fn evaluate(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

// Implement useful traits
impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        if (self.orig == other.orig) && (self.dir == other.dir) {
            return true;
        }
        false
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        let a = m_ray!([0, 0, 0], [1, 0, 0]);
        let b = m_ray!();
        assert_eq!(a, b);

    }
}