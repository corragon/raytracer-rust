use ray::Ray;
use hitable::{hitable, hit_record};
use vector::{Vec3, dot, cross, unit_vector};

pub struct hitable_list {
  pub list_size : i64,
  pub list : Vec<Box<hitable>>,
}

impl hitable for hitable_list {
  fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    false
  }
}