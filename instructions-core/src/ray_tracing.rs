use core::fmt::Debug;
use rand::Rng;
use std::f32::{consts, INFINITY};
use std::fs::File;
use std::io::Write;

pub trait Intersect: Debug {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub trait Normal: Intersect {
    fn normal(&self, point: &Vec3) -> Ray;
    fn material(&self) -> &dyn Material;
}

#[derive(Debug)]
pub struct Hit<'a> {
    point: Vec3,
    distance: f32,
    object: &'a dyn Normal,
}

impl<'a> Hit<'a> {
    pub fn new(point: &Vec3, distance: f32, object: &'a dyn Normal) -> Self {
        Hit {
            point: point.clone(),
            distance,
            object,
        }
    }

    fn normal(&self) -> Ray {
        self.object.normal(&self.point)
    }

    fn collide(&self, ray: &Ray) -> Ray {
        self.object.material().collide(ray, self)
    }

    fn albedo(&self) -> &Vec3 {
        self.object.material().albedo()
    }
}

#[derive(Debug)]
pub struct Intersectables<'a> {
    objects: Vec<&'a dyn Intersect>,
}

impl<'a> Intersectables<'a> {
    pub fn new(objects: Vec<&'a dyn Intersect>) -> Self {
        Intersectables { objects }
    }

    pub fn push(&mut self, object: &'a dyn Intersect) {
        self.objects.push(object);
    }
}

impl<'a> Intersect for Intersectables<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut ret: Option<Hit> = None;
        for i in &self.objects {
            let temp = i.intersect(ray, t_min, t_max);
            //need to check if temp exists and if so if it is closer than we currently have
            match &temp {
                //if a hit
                Some(temp_hit) => {
                    //check if ret is None
                    match (&ret).as_ref() {
                        //if we already had a hit
                        Some(ret_hit) => {
                            //if our existing hit is further than our latest then our latest is our new existing hit
                            if ret_hit.distance > temp_hit.distance {
                                ret = temp;
                            }
                        }
                        //ret is None then we set temp to our new hit
                        None => {
                            ret = temp;
                        }
                    }
                }
                //no hit then don't do anything
                None => {}
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: &Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Sphere {
            center: center.clone(),
            radius,
            material,
        }
    }
    pub fn center(&self) -> &Vec3 {
        &self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<'a> Intersect for Sphere<'a> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let a = 1.0;
        let b = 2.0 * ray.direction().dot(&(ray.origin() - &(self.center)));
        let c = ray.origin().squared_length() - 2.0 * ray.origin().dot(&(self.center))
            + self.center.squared_length()
            - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 1e-5 {
            None
        } else {
            let mut t: f32 = discriminant.sqrt();
            let t1 = -b + t;
            let t2 = -b - t;
            //take closest intersection
            if t1.abs() < t2.abs() {
                t = t1;
            } else {
                t = t2;
            }
            t *= 0.5; //divide by 2 in denom. of quad. formula
            if t < t_min || t > t_max {
                return None;
            }
            Some(Hit::new(&ray.parameterization(t), t, self))
        }
    }
}

impl<'a> Normal for Sphere<'a> {
    fn normal(&self, point: &Vec3) -> Ray {
        Ray::new(point, &(point - &self.center))
    }
    fn material(&self) -> &dyn Material {
        self.material
    }
}

pub trait Material: Debug {
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray;
    fn albedo(&self) -> &Vec3;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
    pub fn new(albedo: &Vec3) -> Self {
        Lambertian {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn collide(&self, ray_in: &Ray, hit: &Hit) -> Ray {
        Ray::from_spherical(
            &hit.point,
            rand::thread_rng().gen_range(0.0..=consts::PI),
            rand::thread_rng().gen_range(0.0..=2.0 * consts::PI),
        )
    }
    fn albedo(&self) -> &Vec3 {
        &self.albedo
    }
}

pub fn color(ray: &Ray, world: &Intersectables, max_hits: i32) -> Vec3 {
    if max_hits == 0 {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
    }
    match world.intersect(ray, 0.001, INFINITY) {
        Some(hit) => {
            return &color(
                &Ray::from_spherical(
                    &hit.point,
                    rand::thread_rng().gen_range(0.0..=consts::PI),
                    rand::thread_rng().gen_range(0.0..=2.0 * consts::PI),
                ),
                world,
                max_hits - 1,
            ) * 0.5;
        }
        _ => {
            let unit_direction = ray.direction().normalize();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return &(&Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3::new(0.5, 0.7, 1.0) * t);
        }
    };
}

// fn main() {
//     let nx = 200;
//     let ny = 100;
//     let ns = 100;
//     let mut data = format!("P3\n{} {} \n255\n", nx, ny);
//     let material1 = Lambertian::new(&Vec3::new(0.8, 0.3, 0.3));
//     let material2 = Lambertian::new(&Vec3::new(0.8, 0.8, 0.0));
//     let material3 = Lambertian::new(&Vec3::new(0.8, 0.6, 0.2));
//     let material4 = Lambertian::new(&Vec3::new(0.8, 0.8, 0.8));
//     let sphere1 = Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, &material1);
//     let sphere2 = Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, &material2);
//     let world = Intersectables::new(vec![&sphere1, &sphere2]);
//     let camera = Camera::new();
//     for j in (0..ny).rev() {
//         for i in 0..nx {
//             let mut col = Vec3::new(0.0, 0.0, 0.0);
//             for s in 0..ns {
//                 let rand: f32 = rand::thread_rng().gen_range(0.0, 1.0);
//                 let u = ((i as f32) + rand) / (nx as f32);
//                 let v = ((j as f32) + rand) / (ny as f32);
//                 let r = camera.get_ray(u, v);
//                 col += &color(&r, &world, 10000);
//             }
//             col /= ns as f32;
//             let ir = col.r();
//             let ig = col.g();
//             let ib = col.b();
//             data.push_str(&format!(
//                 "{} {} {}\n",
//                 ((255.99 * ir) as i32),
//                 ((255.99 * ig) as i32),
//                 ((255.99 * ib) as i32)
//             ));
//         }
//     }
//     let mut f = File::create("test.ppm").expect("Unable to create file");
//     f.write_all(data.as_bytes()).expect("Unable to write data");
// }

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(&(&self.lower_left_corner + &(&(&self.horizontal * u) + &(&self.vertical * v)))
                - &self.origin),
        )
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Box<Vec3>,
    direction: Box<Vec3>,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: Box::new(origin.clone()),
            direction: Box::new(direction.normalize()),
        }
    }
    fn parameterization(&self, t: f32) -> Vec3 {
        &(&(*self.direction) * t) + &(*self.origin)
    }
    fn direction(&self) -> &Vec3 {
        &self.direction
    }
    fn origin(&self) -> &Vec3 {
        &self.origin
    }
    fn from_spherical(origin: &Vec3, phi: f32, theta: f32) -> Self {
        //origin: source of ray
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        Ray {
            origin: Box::new(origin.clone()),
            direction: Box::new(Vec3::from_spherical(1.0, phi, theta)),
        }
    }
}

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(e1: f32, e2: f32, e3: f32) -> Vec3 {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn r(&self) -> f32 {
        self.x
    }
    pub fn g(&self) -> f32 {
        self.y
    }
    pub fn b(&self) -> f32 {
        self.z
    }
    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    fn squared_length(&self) -> f32 {
        self.dot(self)
    }
    fn normalize(&self) -> Vec3 {
        self / self.length()
    }
    fn _normalize(&mut self) {
        *self = self.normalize();
    }
    fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }
    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y - other.x,
        }
    }
    fn dot(&self, other: &Vec3) -> f32 {
        (self * other).sum()
    }
    fn project(&self, onto: &Vec3) -> Vec3 {
        onto * (self.dot(onto) / onto.squared_length())
    }
    fn rotate(&self, phi: f32, theta: f32) -> Vec3 {
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_theta, cos_theta) = theta.sin_cos();
        let col1 = Vec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi);
        let col2 = Vec3::new(-sin_theta * sin_phi, cos_theta * sin_phi, 0.0);
        let col3 = Vec3::new(cos_theta * cos_phi, sin_theta * cos_phi, -sin_phi);
        &(&(&col1 * self.x) + &(&col2 * self.y)) + &(&col3 * self.z)
    }

    fn from_spherical(radius: f32, phi: f32, theta: f32) -> Self {
        //radius ρ in [0, infinity)
        //phi φ in [0, pi] indicates a deviation in radians from the +z axis
        //theta in [0, 2pi] indicates a deviation from the +x axis in the x-y plane
        let sin_phi = phi.sin();
        Vec3::new(
            radius * sin_phi * theta.cos(),
            radius * sin_phi * theta.sin(),
            radius * phi.cos(),
        )
    }
}

impl Add<f32> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<f32> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        let temp = 1.0 / rhs;
        self * temp
    }
}

impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let temp = 1.0 / rhs;
        *self *= temp;
    }
}

impl DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: &Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
