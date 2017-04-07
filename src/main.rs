use std::io::{Write, BufWriter};
use std::fs::File;
mod vector;
use vector::Vec3;

fn main() {
  let nx = 300;
  let ny = 200;
  let data = format!("{}\n{} {}\n{}\n", "P3", nx, ny, 255);
  let f = File::create("target/image.ppm").expect("Unable to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes()).expect("Unable to write data");

  for j in (0..ny).rev() {
    for i in 0..nx {
      let color = Vec3 { i: i as f64 / nx as f64, j: j as f64 / ny as f64, k: 0.5 };

      let ir = (255.99*color.i) as i32;
      let ig = (255.99*color.j) as i32;
      let ib = (255.99*color.k) as i32;
      f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write data");
    }
  }
}