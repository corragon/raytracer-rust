use vector::Vec3;
use ray::Ray;

pub struct hit_record {
  pub t : f64,
  pub p : Vec3,
  pub normal : Vec3,
}

trait hitable {
  fn hit(r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool
}