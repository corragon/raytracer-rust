use ray::Ray;
use hitable::{Hitable, hit_record};
use vector::{Vec3, dot, cross, unit_vector};

pub struct Sphere {
  pub center : Vec3,
  pub radius : f64,
}

impl Sphere {
  pub fn new(cen: Vec3, r: f64) -> Sphere {
    Sphere { center : cen, radius : r }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    let oc = r.origin() - self.center;
    let a = dot(r.direction(), r.direction());
    let b = dot(oc, r.direction());
    let c = dot(oc, oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;

    if discriminant > 0.0 {
      let mut temp = (-b - (b * b - a * c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
      temp = (-b + (b * b - a * c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
    }
    return false;
  }
}