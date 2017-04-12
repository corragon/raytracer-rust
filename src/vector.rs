use std::ops::{
  Add, AddAssign,
  Sub, SubAssign,
  Mul, MulAssign,
  Div, DivAssign,
  Index, IndexMut,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
  pub i : f64,
  pub j : f64,
  pub k : f64,
}

impl Vec3 {
  pub fn new(first : f64, second : f64, third : f64) -> Vec3 {
    Vec3 { i: first, j: second, k: third }
  }
  pub fn x(&self) -> f64 { self.i }
  pub fn y(&self) -> f64 { self.j }
  pub fn z(&self) -> f64 { self.k }
  pub fn r(&self) -> f64 { self.i }
  pub fn g(&self) -> f64 { self.j }
  pub fn b(&self) -> f64 { self.k }
  pub fn length(&self) -> f64 {
    (self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
  }
  pub fn squared_length(&self) -> f64 {
    self.i * self.i + self.j * self.j + self.k * self.k
  }
  pub fn dot(&self, other: Vec3) -> f64 {
    dot(*self, other)
  }
  pub fn cross(&self, other: Vec3) -> Vec3 {
    cross(*self, other)
  }
  pub fn unit_vector(&self) -> Vec3 {
    unit_vector(*self)
  }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
  v1.i * v2.i + v1.j * v2.j + v1.k * v2.k
}
pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
  return Vec3 {
    i: v1.j * v2.k - v1.k * v2.j,
    j: -(v1.i * v2.k - v1.k * v2.i),
    k: v1.i * v2.j - v1.j * v2.i,
  };
}
pub fn unit_vector(v1: Vec3) -> Vec3 {
  return v1 / v1.length()
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

impl Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {i: self.i * rhs.i, j: self.j * rhs.j, k: self.k * rhs.k}
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, scalar: f64) -> Vec3 {
    Vec3 {i: self.i * scalar, j: self.j * scalar, k: self.k * scalar}
  }
}

impl MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, rhs: Vec3) {
    *self = Vec3 {
      i: self.i * rhs.i,
      j: self.j * rhs.j,
      k: self.k * rhs.k,
    }
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, scalar: f64) {
    *self = Vec3 {
      i: self.i * scalar,
      j: self.j * scalar,
      k: self.k * scalar,
    }
  }
}

impl Div<Vec3> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: Vec3) -> Vec3 {
    Vec3 {i: self.i / rhs.i, j: self.j / rhs.j, k: self.k / rhs.k}
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, scalar: f64) -> Vec3 {
    Vec3 {i: self.i / scalar, j: self.j / scalar, k: self.k / scalar}
  }
}

impl DivAssign<Vec3> for Vec3 {
  fn div_assign(&mut self, rhs: Vec3) {
    *self = Vec3 {
      i: self.i / rhs.i,
      j: self.j / rhs.j,
      k: self.k / rhs.k,
    }
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, scalar: f64) {
    *self = Vec3 {
      i: self.i / scalar,
      j: self.j / scalar,
      k: self.k / scalar,
    }
  }
}

impl Index<i32> for Vec3 {
  type Output = f64;
  fn index(&self, idx: i32) -> &f64 {
    match idx {
      0 => &self.i,
      1 => &self.j,
      2 => &self.k,
      _ => panic!("index out of range: {}", idx),
    }
  }
}

impl IndexMut<i32> for Vec3 {
  fn index_mut<'a>(&'a mut self, idx: i32) -> &'a mut f64 {
    match idx {
      0 => &mut self.i,
      1 => &mut self.j,
      2 => &mut self.k,
      _ => panic!("index out of range: {}", idx),
    }
  }
}

#[cfg(test)]
mod should {
  use super::{Vec3, dot, cross, unit_vector};
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
  #[test]
  fn multiply_by_vec3_elementwise() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};
    let product = v1 * v2;
    assert_eq!(product.i, 6.0);
    assert_eq!(product.j, 6.0);
    assert_eq!(product.k, 6.0);
  }
  #[test]
  fn multiply_by_vec3_assign_elementwise() {
    let mut v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};
    v1 *= v2;
    assert_eq!(v1.i, 6.0);
    assert_eq!(v1.j, 6.0);
    assert_eq!(v1.k, 6.0);
  }
  #[test]
  fn multiply_by_f64() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let product = v1 * 3.0;

    assert_eq!(product.i, 6.0);
    assert_eq!(product.j, 6.0);
    assert_eq!(product.k, 6.0);
  }
  #[test]
  fn multiply_by_f64_assign() {
    let mut v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    v1 *= 3.0;

    assert_eq!(v1.i, 6.0);
    assert_eq!(v1.j, 6.0);
    assert_eq!(v1.k, 6.0);
  }
  #[test]
  fn divide_by_vec3_elementwise() {
    let v1 = Vec3 { i: 6.0, j: 6.0, k: 6.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};
    let quotient = v1 / v2;
    assert_eq!(quotient.i, 2.0);
    assert_eq!(quotient.j, 2.0);
    assert_eq!(quotient.k, 2.0);
  }
  #[test]
  fn divide_by_vec3_assign_elementwise() {
    let mut v1 = Vec3 { i: 6.0, j: 6.0, k: 6.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};
    v1 /= v2;
    assert_eq!(v1.i, 2.0);
    assert_eq!(v1.j, 2.0);
    assert_eq!(v1.k, 2.0);
  }
  #[test]
  fn divide_by_f64() {
    let v1 = Vec3 { i: 6.0, j: 6.0, k: 6.0};
    let quotient = v1 / 3.0;

    assert_eq!(quotient.i, 2.0);
    assert_eq!(quotient.j, 2.0);
    assert_eq!(quotient.k, 2.0);
  }
  #[test]
  fn divide_by_f64_assign() {
    let mut v1 = Vec3 { i: 6.0, j: 6.0, k: 6.0};
    v1 /= 3.0;

    assert_eq!(v1.i, 2.0);
    assert_eq!(v1.j, 2.0);
    assert_eq!(v1.k, 2.0);
  }
  #[test]
  fn allow_index_access() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};

    assert_eq!(v1[0], 2.0);
    assert_eq!(v1[1], 2.0);
    assert_eq!(v1[2], 2.0);
  }
  #[test]
  #[should_panic(expected = "index out of range: 9")]
  fn panic_on_bad_index() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};

    assert_eq!(v1[9], 2.0);
  }
  #[test]
  fn allow_index_mut_access() {
    let mut v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    v1[0] = 9.0;

    assert_eq!(v1[0], 9.0);
    assert_eq!(v1[1], 2.0);
    assert_eq!(v1[2], 2.0);
  }




  #[test]
  fn construct_with_new() {
    let v1 = Vec3::new(4.0, 5.0, 6.0);

    assert_eq!(v1[0], 4.0);
    assert_eq!(v1[1], 5.0);
    assert_eq!(v1[2], 6.0);
  }
  #[test]
  fn calculate_length() {
    let v1 = Vec3 { i: 4.0, j: 4.0, k: 2.0};

    assert_eq!(v1.length(), 6.0);
  }
  #[test]
  fn calculate_length_squared() {
    let v1 = Vec3 { i: 4.0, j: 4.0, k: 2.0};

    assert_eq!(v1.squared_length(), 36.0);
  }
  #[test]
  fn calculate_length_with_some_zeros() {
    let v2 = Vec3 { i: 4.0, j: 0.0, k: 0.0};

    assert_eq!(v2.length(), 4.0);
  }
  #[test]
  fn calculate_length_for_zero_vector() {
    let v2 = Vec3 { i: 0.0, j: 0.0, k: 0.0};

    assert_eq!(v2.length(), 0.0);
  }
  #[test]
  fn calculate_dot_product() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};

    let product = v1.dot(v2);

    assert_eq!(product, 18.0);
  }
  #[test]
  fn calculate_cross_product() {
    let v1 = Vec3 { i: 1.0, j: 0.0, k: 0.0};
    let v2 = Vec3 { i: 0.0, j: 1.0, k: 0.0};

    let product = v1.cross(v2);

    assert_eq!(product[0], 0.0);
    assert_eq!(product[1], 0.0);
    assert_eq!(product[2], 1.0);
  }
  #[test]
  fn calculate_unit_vector_simple() {
    let v1 = Vec3 { i: 4.0, j: 0.0, k: 0.0};

    let product = v1.unit_vector();

    assert_eq!(product[0], 1.0);
    assert_eq!(product[1], 0.0);
    assert_eq!(product[2], 0.0);
  }
  #[test]
  fn calculate_unit_vector_multple() {
    let v1 = Vec3 { i: 4.0, j: 4.0, k: 2.0};

    let product = v1.unit_vector();

    assert_eq!(product[0], 4.0 / 6.0);
    assert_eq!(product[1], 4.0 / 6.0);
    assert_eq!(product[2], 2.0 / 6.0);
  }
  #[test]
  fn calculate_dot_product_static() {
    let v1 = Vec3 { i: 2.0, j: 2.0, k: 2.0};
    let v2 = Vec3 { i: 3.0, j: 3.0, k: 3.0};

    let product = dot(v1, v2);

    assert_eq!(product, 18.0);
  }
  #[test]
  fn calculate_cross_product_static() {
    let v1 = Vec3 { i: 1.0, j: 0.0, k: 0.0};
    let v2 = Vec3 { i: 0.0, j: 1.0, k: 0.0};

    let product = cross(v1, v2);

    assert_eq!(product[0], 0.0);
    assert_eq!(product[1], 0.0);
    assert_eq!(product[2], 1.0);
  }
  #[test]
  fn calculate_unit_vector_static_simple() {
    let v1 = Vec3 { i: 4.0, j: 0.0, k: 0.0};

    let product = unit_vector(v1);

    assert_eq!(product[0], 1.0);
    assert_eq!(product[1], 0.0);
    assert_eq!(product[2], 0.0);
  }
  #[test]
  fn calculate_unit_vector_static_multple() {
    let v1 = Vec3 { i: 4.0, j: 4.0, k: 2.0};

    let product = unit_vector(v1);

    assert_eq!(product[0], 4.0 / 6.0);
    assert_eq!(product[1], 4.0 / 6.0);
    assert_eq!(product[2], 2.0 / 6.0);
  }
  #[test]
  fn have_color_access_functions() {
    let v1 = Vec3 { i: 1.0, j: 2.0, k: 3.0};

    assert_eq!(v1.r(), 1.0);
    assert_eq!(v1.g(), 2.0);
    assert_eq!(v1.b(), 3.0);
  }
  #[test]
  fn have_coordinate_access_functions() {
    let v1 = Vec3 { i: 1.0, j: 2.0, k: 3.0};

    assert_eq!(v1.x(), 1.0);
    assert_eq!(v1.y(), 2.0);
    assert_eq!(v1.z(), 3.0);
  }
}