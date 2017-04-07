use std::io::{Write, BufWriter};
use std::fs::File;
mod vector;

fn main() {
  let v = vector::Vec3 { c1: 200f64, c2: 200f64, c3: 255f64 };
  let nx = 200;
  let ny = 200;
  let data = format!("{}\n{} {}\n{}\n", "P3", v.c1, v.c2, v.c3);
  let f = File::create("target/image.ppm").expect("Unable to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes()).expect("Unable to write data");

  for j in (0..ny).rev() {
    for i in 0..nx {
      let r : f32 = i as f32 / nx as f32;
      let g : f32 = j as f32 / ny as f32;
      let b : f32 = 0.7;
      let ir = (255.99*r) as i32;
      let ig = (255.99*g) as i32;
      let ib = (255.99*b) as i32;
      f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write data");
    }
  }
}