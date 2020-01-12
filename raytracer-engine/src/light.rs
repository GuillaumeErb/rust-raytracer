use crate::color::Color;
use crate::geometry::Point3;
use crate::geometry::Vector3;

#[derive(Debug)]
pub enum Light {
    DirectionalLight(DirectionalLight),
    Point3Light(Point3Light),
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,
    pub color: Color,
}

#[derive(Debug)]
pub struct Point3Light {
    pub origin: Point3,
    pub intensity: f64,
    pub color: Color,
}

#[derive(Debug)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f64,
}

impl Light {
    pub fn get_direction(&self, point: &Point3) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref light) => light.direction,
            Light::Point3Light(ref light) => (&light.origin - &point).normalize(),
        }
    }

    pub fn get_intensity(&self) -> f64 {
        match *self {
            Light::DirectionalLight(ref light) => light.intensity,
            Light::Point3Light(ref light) => light.intensity,
        }
    }

    pub fn get_color(&self) -> Color {
        match *self {
            Light::DirectionalLight(ref light) => light.color,
            Light::Point3Light(ref light) => light.color,
        }
    }
}
