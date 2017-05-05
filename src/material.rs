use ray::Ray;
use vector::Vec3;
use hitable::HitRecord;

pub trait Material {
  fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}