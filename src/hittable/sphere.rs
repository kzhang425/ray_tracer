use super::*;
use crate::vec3::dot;

pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Self {
        Self {
            radius: r,
            center: cen,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Rec) -> bool {
        let oc = r.origin() - self.center;

        // a, b, c are basically coefficients of a quadratic equation
        let a = r.direction().length().powi(2);
        let half_b = dot(r.direction(), oc);
        let c = dot(oc, oc) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            *rec = None;
            return false;
        }

        let sqrt_disc = discriminant.sqrt();
        let mut root = (-half_b - sqrt_disc) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_disc) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        // Write data to the preallocated Rec
        *rec = Some(HitRecord::new(r.at(root), (r.at(root) - self.center) / self.radius, root));

        true

    }
}