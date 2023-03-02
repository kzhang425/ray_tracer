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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Rec {
        let oc = r.origin() - self.center;

        // a, b, c are basically coefficients of a quadratic equation
        let a = r.direction().length().powi(2);
        let half_b = dot(r.direction(), oc);
        let c = dot(oc, oc) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let mut root = (-half_b - sqrt_disc) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_disc) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        // Determine whether the hit happens outside the sphere or inside the sphere
        let outward_normal = (r.at(root) - self.center) / self.radius;
        let mut rec = HitRecord::new(r.at(root), outward_normal, root, true);

        // is_front_face can change and correct for the front face field
        rec.set_face_normal(r, &outward_normal);

        // Write data to the preallocated Rec
        // The normal makes it such that it is always pointing outwards of the circle, regardless of ray direction
        let rec = Some(rec);

        rec

    }
}