#![deny(clippy::all)]
#![forbid(unsafe_code)]

use cgmath::{dot, Vector3};
use cgmath::num_traits::Pow;

pub fn write_color(vec: Vector3<f32>) -> [u8; 4]{
    return [
        (vec.x * 256.) as u8,
        (vec.y * 256.) as u8,
        (vec.z * 256.) as u8,
        (0xff) as u8];
}

pub fn ray_color(r: Ray) -> Vector3<f32>{
    let t = hit_sphere(Vector3::new(0., 0., -1.), 0.5, &r);
    if t > 0. {
        let n = unit_vector(r.at(t) - Vector3::new(0., 0., -1.));
        return 0.5 * Vector3::new(n.x + 1., n.y + 1., n.z + 1.);
    }
    let unit_direction: Vector3<f32> = unit_vector(r.dir);
    let t: f32 = 0.5 * (unit_direction.y + 1.);
    return (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.0);
}

pub fn unit_vector(v: Vector3<f32>) -> Vector3<f32>{
    let mag: f32 = (v.x.pow(2.) + v.y.pow(2.) + v.z.pow(2.)).sqrt();
    return v / mag;
}

pub struct Ray{
    orig: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn new(o: Vector3<f32>, d: Vector3<f32>) -> Self {
        Self{
            orig: o,
            dir: d,
        }
    }

    fn at(&self, t: f32) -> Vector3<f32>{
        return self.orig + (t * self.dir);
    }
}

fn hit_sphere(center: Vector3<f32>, radius: f32, r: &Ray) -> f32 {
    let oc = r.orig - center;
    let a = dot(r.dir, r.dir);
    let b = 2. * dot(oc, r.dir);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    return if discriminant < 0. {
        -1.
    } else {
        (-b - discriminant.sqrt()) / (2. * a)
    }
}