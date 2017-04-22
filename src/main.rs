use std::fs::File;

extern crate rand;
use rand::Rng;

extern crate image;
use image::{
  ImageBuffer
};

mod vector;
mod ray;
mod sphere;
mod hitable;
mod hitable_list;
mod camera;
mod material;

use vector::{Vec3, dot, unit_vector};
use ray::Ray;
use sphere::{Sphere};
use hitable::{Hitable, HitRecord};
use hitable_list::HitableList;
use camera::Camera;


fn color<T: Hitable>(ray : Ray, world : &T) -> Vec3 {
  let mut rec = HitRecord::new(0.0, Vec3::origin(), Vec3::origin());
  if world.hit(ray, 0.0, std::f64::MAX, &mut rec) {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    return color(Ray::new(rec.p, target - rec.p), world) * 0.5
  }
  else {
    let unit_direction = unit_vector(ray.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
  }
}

fn random_in_unit_sphere() -> Vec3 {
  let mut p : Vec3;
  let mut rng = rand::thread_rng();
  loop {
    p = Vec3::new(rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>()) * 2.0 - Vec3::all(1.0);
    if dot(p, p) < 1.0 { break; }
  }
  p
}

#[allow(dead_code)]
fn random<T: Hitable>(i : i32, j : i32, nx : i32, ny : i32, cam : Camera, world : &T, rng : &mut rand::ThreadRng) -> Vec3 {
  let sample_size = 100;
  let mut samples = 0;
  let mut col = Vec3::origin();
  loop {
    let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
    let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
    let r = cam.get_ray(u, v);

    col += color(r, world);
    samples += 1;
    if samples >= sample_size { break; }
  }

  col /= sample_size as f64;
  return col;
}

fn stratified<T: Hitable>(i : i32, j : i32, nx : i32, ny : i32, cam : Camera, world : &T, rng : &mut rand::ThreadRng) -> Vec3 {
  let mut col = Vec3::origin();
  let grid_size = 5;
  let step = 1.0 / grid_size as f64;
  let low_i = i as f64 - 0.5;
  let low_j = j as f64 - 0.5;

  for ii in 0..grid_size {
    for jj in 0..grid_size {
      let u = ((low_i + ii as f64 * step) + rng.gen_range(0.0, step)) / nx as f64;
      let v = ((low_j + jj as f64 * step) + rng.gen_range(0.0, step)) / ny as f64;
      let r = cam.get_ray(u, v);

      col += color(r, world);
    }
  }

  col /= (grid_size * grid_size) as f64;
  return col;
}

fn main() {
  let nx = 200;
  let ny = 100;
  let mut img = ImageBuffer::new(nx as u32, ny as u32);
  let mut f = File::create("target/image.png").expect("Unable to create file");

  let upper_left_corner = Vec3::new(-2.0, 1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  let cam = Camera::new(upper_left_corner, vertical, horizontal, origin);

  let mut list : Vec<Box<Hitable>> = Vec::new();
  list.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
  list.push(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));
  let world = HitableList::new(list);

  let mut rng = rand::thread_rng();

  for (x, y, pixel) in img.enumerate_pixels_mut() {
    let mut col = stratified(x as i32, y as i32, nx, ny, cam, &world, &mut rng);
    col = col.sqrt();
    *pixel = image::Rgb([(col.i * 255.0) as u8, (col.j * 255.0) as u8, (col.k * 255.0) as u8]);
  }

  let _ = image::ImageRgb8(img).save(&mut f, image::PNG);
}