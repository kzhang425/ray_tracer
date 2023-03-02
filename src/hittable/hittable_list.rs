use super::*;

/// The implementation of World is really to superimpose objects that can obstruct a ray. It will check all the cases in the world
/// and then render accordingly. This will indeed return the closest point out of all objects we place into our world.
pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Rec {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        // Since the type itself is a vector, it can be iterated through
        for obj in self {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}