use crate::Color;
use crate::Intersection;
use crate::LambertMaterial;
use crate::Ray;
use crate::Scene;

#[derive(Debug)]
pub enum Material {
    ConstantMaterial(ConstantMaterial),
    LambertMaterial(LambertMaterial),
}

#[derive(Debug)]
pub struct ConstantMaterial {
    pub color: Color,
}

impl Material {
    pub fn render_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
        match *self {
            Material::ConstantMaterial(ref m) => m.color,
            Material::LambertMaterial(ref m) => m.render_color(ray, intersection, scene),
        }
    }
}
