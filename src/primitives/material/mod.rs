use super::{color::Color, objects::HitRecord, ray::Ray};

mod metal;

pub trait Material {
    fn scatter(ray: &Ray, hit_rec: &HitRecord, attenuation: &Color, scatter: &Ray) -> Option<HitRecord>;
}
