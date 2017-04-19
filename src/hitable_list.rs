use ray::Ray;
use hitable::{Hitable, HitRecord};
use vector::Vec3;

pub struct HitableList {
  pub list_size : i64,
  pub list : Vec<Box<Hitable>>,
}

impl HitableList {
  pub fn new(l : Vec<Box<Hitable>>) -> HitableList {
    HitableList { list_size : l.len() as i64, list: l }
  }
}

impl Hitable for HitableList {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new(0.0, Vec3::origin(), Vec3::origin());
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