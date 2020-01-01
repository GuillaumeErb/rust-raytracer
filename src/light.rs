use crate::Color;
use crate::Point;
use crate::Vector3;

#[derive(Debug)]
pub enum Light {
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,
    pub color: Color,
}

#[derive(Debug)]
pub struct PointLight {
    pub origin: Point,
    pub intensity: f64,
    pub color: Color,
}

#[derive(Debug)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f64,
}

impl Light {
    pub fn get_direction(&self, point: &Point) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref light) => light.direction,
            Light::PointLight(ref light) => (&light.origin - &point).normalize(),
        }
    }

    pub fn get_intensity(&self) -> f64 {
        match *self {
            Light::DirectionalLight(ref light) => light.intensity,
            Light::PointLight(ref light) => light.intensity,
        }
    }

    pub fn get_color(&self) -> Color {
        match *self {
            Light::DirectionalLight(ref light) => light.color,
            Light::PointLight(ref light) => light.color,
        }
    }
}
