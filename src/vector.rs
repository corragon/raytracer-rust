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
