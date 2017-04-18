use vector::Vec3;
use ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
  pub upper_left_corner : Vec3,
  pub vertical : Vec3,
  pub horizontal : Vec3,
  pub origin : Vec3,
}

impl Camera {
  pub fn new(upper : Vec3, vert : Vec3, horiz : Vec3, orig: Vec3) -> Camera {
    Camera { 
      upper_left_corner: upper,
      vertical: vert,
      horizontal: horiz,
      origin: orig,
    }
  }
  pub fn get_ray(&self, u : f64, v : f64) -> Ray {
    Ray::new(self.origin, self.upper_left_corner + self.horizontal * u - self.vertical * v - self.origin)
  }
}

