use std::io::{Write, BufWriter};
use std::fs::File;
mod vector;
use vector::{Vec3, dot, cross, unit_vector};
mod ray;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
  let oc = ray.origin() - center;
  let a: f64 = dot(ray.direction(), ray.direction());
  let b: f64 = dot(oc, ray.direction()) * 2.0;
  let c: f64 = dot(oc, oc) - radius * radius;
  let discriminant = b * b - 4.0 * a * c;
  if discriminant < 0.0 {
    return -1.0
  }
  else {
    return (-b - discriminant.sqrt()) / (2.0 * a);
  }
}

fn color(ray : Ray) -> Vec3 {
  let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
  if t > 0.0 {
    let n = unit_vector(ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
    return (Vec3::new(n.x()+1.0, n.y()+1.0, n.z()+1.0)) * 0.5;
  }

  let unit_direction = ray.direction().unit_vector();
  let t = 0.5 * (unit_direction.y() + 1.0);
  return Vec3::new(1.0,1.0,1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t;
}

fn main() {
  let nx = 200;
  let ny = 100;
  let data = format!("{}\n{} {}\n{}\n", "P3", nx, ny, 255);
  let f = File::create("target/image.ppm").expect("Unable to create file");
  let mut f = BufWriter::new(f);
  f.write_all(data.as_bytes()).expect("Unable to write data");

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = i as f64 / nx as f64;
      let v = j as f64 / ny as f64;
      let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
      let col = color(r);

      let ir = (255.99*col.r()) as i32;
      let ig = (255.99*col.g()) as i32;
      let ib = (255.99*col.b()) as i32;
      f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write data");
    }
  }
}