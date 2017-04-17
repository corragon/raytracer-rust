use vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  pub a : Vec3,
  pub b : Vec3,
}

impl Ray {
  pub fn new(a: Vec3, b: Vec3) -> Ray {
    Ray { a: a, b: b}
  }
  pub fn origin(&self) -> Vec3 {
    self.a
  }
  pub fn direction(&self) -> Vec3 {
    self.b
  }
  pub fn point_at_parameter(&self, t: f64) -> Vec3 {
    self.a + self.b * t
  }
}

#[cfg(test)]
mod should {
  use super::Ray;
  use vector::Vec3;

  #[test]
  fn construct_with_new() {
    let v1 = Vec3 { i: 1.0, j: 1.0, k: 1.0};
    let v2 = Vec3 { i: 2.0, j: 2.0, k: 2.0};

    let r = Ray::new(v1, v2);

    assert_eq!(r.origin(), v1);
    assert_eq!(r.direction(), v2);
  }
  #[test]
  fn calculate_point() {
    let v1 = Vec3 { i: 0.0, j: 0.0, k: 0.0};
    let v2 = Vec3 { i: 1.0, j: 1.0, k: 1.0};

    let r = Ray::new(v1, v2);

    assert_eq!(r.point_at_parameter(1.0), v2);
    assert_eq!(r.point_at_parameter(7.0), v2 * 7.0);
  }
}