use crate::Color;
use crate::Vector3;

#[derive(Debug)]
pub enum Light {
    DirectionalLight(DirectionalLight),
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn get_direction(&self) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref light) => light.direction,
        }
    }

    pub fn get_intensity(&self) -> f64 {
        match *self {
            Light::DirectionalLight(ref light) => light.intensity,
        }
    }

    pub fn get_color(&self) -> Color {
        match *self {
            Light::DirectionalLight(ref light) => light.color,
        }
    }
}
