use ray::Ray;
use hitable::{Hitable, hit_record};
use vector::Vec3;

pub struct Hitable_list {
  pub list_size : i64,
  pub list : Vec<Box<Hitable>>,
}

impl Hitable_list {
  pub fn new(l : Vec<Box<Hitable>>) -> Hitable_list {
    Hitable_list { list_size : l.len() as i64, list: l }
  }
}

impl Hitable for Hitable_list {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    let mut temp_rec = hit_record::new(0.0, Vec3::origin(), Vec3::origin());
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for item in self.list.iter() {
      if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        rec.t = temp_rec.t;
        rec.p = temp_rec.p;
        rec.normal = temp_rec.normal;
      }
    }
    return hit_anything;
  }
}