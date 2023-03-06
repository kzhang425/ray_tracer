use super::vec3::{Vec3, Point3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// Makes a new default camera, no parameters
    pub fn new() -> Self {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin: orig,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc
        }
    }

    /// Generates a ray given u and v values. These are relative to the camera frame and is used with iteration to generate all possible rays.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}