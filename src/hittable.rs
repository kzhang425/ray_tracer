pub(crate) mod sphere;
pub(crate) mod hittable_list;

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

/// This struct serves as a second output to a class that will inherit the trait Hittable. The trait only returns a bool but
/// also should write the calculated results from the particular ray instance to this struct. However, do not use this struct directly
/// as the trait can potentially not yield any HitRecords.
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
        match self.front_face {
            true => {
                self.normal = *outward_normal;
            },
            _ => {
                self.normal = *outward_normal * (-1.0);
            },
        }
    }
}

/// Rec is a HitRecord wrapped with an Option. It is a way to represent the nullable type that may be returned after implementing the
/// Hittable trait.
pub type Rec = Option<HitRecord>;

/// ### Trait Hittable
/// This trait serves as a form of inheritance to other objects that we may try to render. The purpose of the &mut Rec is to cover the
/// case that the record cannot be calculated.
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Rec;
}