use ray::Ray;
use vector::Vec3;
use hitable::hit_record;

pub trait Material {
  fn scatter(&self, r: &Ray, rec: &hit_record, attenuation: &Vec3, scattered: &Ray) -> bool;
}