use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub i : f64,
  pub j : f64,
  pub k : f64,
}

impl Vec3 {
  pub fn length(&self) -> f64 {
    (self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, other: Vec3) -> Vec3 {
      Vec3 {i: self.i + other.i, j: self.j + other.j, k: self.k + other.k}
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Vec3) {
    *self = Vec3 {
      i: self.i + other.i,
      j: self.j + other.j,
      k: self.k + other.k,
    };
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
      Vec3 {i: self.i - other.i, j: self.j - other.j, k: self.k - other.k}
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Vec3) {
    *self = Vec3 {
      i: self.i - other.i,
      j: self.j - other.j,
      k: self.k - other.k,
    }
  }
}

impl Mul for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {i: self.i * rhs.i, j: self.j * rhs.j, k: self.k * rhs.k}
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Vec3) {
    *self = Vec3 {
      i: self.i * rhs.i,
      j: self.j * rhs.j,
      k: self.k * rhs.k,
    }
  }
}

#[cfg(test)]
mod should {
  use super::Vec3;
  #[test]
  fn add() {
    let v1 = Vec3 { i: 1.0, j: 1.0, k: 1.0};
    let v2 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let sum = v1 + v2;
    assert_eq!(sum.i, 3.0);
    assert_eq!(sum.j, 3.0);
    assert_eq!(sum.k, 3.0);
  }
  #[test]
  fn add_assign() {
    let mut v1 = Vec3 { i: 1.0, j: 1.0, k: 1.0};
    let v2 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    v1 += v2;
    assert_eq!(v1.i, 3.0);
    assert_eq!(v1.j, 3.0);
    assert_eq!(v1.k, 3.0);
  }
  #[test]
  fn subtract() {
    let v1 = Vec3 { i: 1.0, j: 1.0, k: 1.0};
    let v2 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let diff = v1 - v2;
    assert_eq!(diff.i, -1.0);
    assert_eq!(diff.j, -1.0);
    assert_eq!(diff.k, -1.0);
  }
  #[test]
  fn subtract_assign() {
    let mut v1 = Vec3 { i: 1.0, j: 1.0, k: 1.0};
    let v2 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    v1 -= v2;
    assert_eq!(v1.i, -1.0);
    assert_eq!(v1.j, -1.0);
    assert_eq!(v1.k, -1.0);
  }
}