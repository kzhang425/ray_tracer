mod sphere;

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

/// This struct serves as a second output to a class that will inherit the trait Hittable. The trait only returns a bool but
/// also should write the calculated results from the particular ray instance to this struct. However, do not use this struct directly
/// as the trait can potentially not yield any HitRecords.
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            p,
            normal,
            t,
        }
    }
}

/// Rec is a HitRecord wrapped with an Option. It is a way to represent the nullable type that may be returned after implementing the
/// Hittable trait.
pub type Rec = Option<HitRecord>;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Rec) -> bool;
}