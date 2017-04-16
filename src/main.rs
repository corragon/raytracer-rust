use std::io::{Write, BufWriter};
use std::fs::File;

extern crate rand;
use rand::Rng;

mod vector;
mod ray;
mod sphere;
mod hitable;
mod hitable_list;
mod camera;

use vector::{Vec3, dot, cross, unit_vector};
use ray::Ray;
use sphere::{Sphere};
use hitable::{Hitable, hit_record};
use hitable_list::Hitable_list;
use camera::Camera;


fn color<T: Hitable>(ray : Ray, world : &T) -> Vec3 {
  let mut rec = hit_record::new(0.0, Vec3::origin(), Vec3::origin());
  if world.hit(ray, 0.0, std::f64::MAX, &mut rec) {
    return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
  }
  else {
    let unit_direction = unit_vector(ray.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
  }
}

fn main() {
  let nx = 200;
  let ny = 100;
  let ns = 100;
  let data = format!("{}\n{} {}\n{}\n", "P3", nx, ny, 255);
  let f = File::create("target/image.ppm").expect("Unable to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes()).expect("Unable to write data");

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  let cam = Camera::new(lower_left_corner, vertical, horizontal, origin);

  let mut list : Vec<Box<Hitable>> = Vec::new();
  list.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
  list.push(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));
  let world = Hitable_list::new(list);

  let mut rng = rand::thread_rng();

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::origin();
      for s in 0..ns {
        let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
        let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
        let r = cam.get_ray(u, v);

        let p = r.point_at_parameter(2.0);
        col += color(r, &world);
      }

      col /= ns as f64;

      let ir = (255.99*col.r()) as i32;
      let ig = (255.99*col.g()) as i32;
      let ib = (255.99*col.b()) as i32;
      f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write data");
    }
  }
}