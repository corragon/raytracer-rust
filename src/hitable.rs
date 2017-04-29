use vector::Vec3;
use ray::Ray;

pub struct HitRecord {
  pub time : f64,
  pub point : Vec3,
  pub normal : Vec3,
}

impl HitRecord {
  pub fn new(t : f64, p : Vec3, norm : Vec3) -> HitRecord {
    HitRecord { time: t, point: p, normal: norm }
  }
}

pub trait Hitable {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}