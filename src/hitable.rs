use vector::Vec3;
use ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
  pub t : f64,
  pub p : Vec3,
  pub normal : Vec3,
}

impl HitRecord {
  pub fn new(time : f64, point : Vec3, norm : Vec3) -> HitRecord {
    HitRecord { t: time, p: point, normal: norm }
  }
}

pub trait Hitable {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}