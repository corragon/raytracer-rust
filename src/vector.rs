  pub struct Vec3 {
    pub c1 : f64,
    pub c2 : f64,
    pub c3 : f64,
  }

  impl Vec3 {
    pub fn length(&self) -> f64 {
      (self.c1 * self.c1 + self.c2 * self.c2 + self.c3 * self.c3).sqrt()
    }
  }
