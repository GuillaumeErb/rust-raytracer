use crate::is_in_shadow;
use crate::Color;
use crate::Intersection;
use crate::LambertMaterial;
use crate::Ray;
use crate::Scene;
use crate::BLACK;
use crate::PI;

#[derive(Debug)]
pub enum Material {
    ConstantMaterial(ConstantMaterial),
    LambertMaterial(LambertMaterial),
}

#[derive(Debug)]
pub struct ConstantMaterial {
    pub color: Color,
}

#[derive(Debug)]
pub struct PhongMaterial {
    color: Color,
    specular_reflection: f64,
    diffuse_reflection: f64,
    ambient_reflection: f64,
    shininess: f64,
}

impl Material {
    pub fn render_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
        let point_precise = ray.origin.add(&ray.direction.times(intersection.distance));
        let normal = intersection.object.geometry.get_normal(&point_precise);
        let point = point_precise.add(&normal.times(1e-6));

        let mut rendered_color = BLACK;
        for light in &scene.lights {
            if is_in_shadow(&point, &light, scene) {
                continue;
            }
            match *self {
                Material::ConstantMaterial(ref m) => rendered_color = &rendered_color + &m.color,
                Material::LambertMaterial(ref m) => {
                    let mut ratio = normal.dot(&light.get_direction().times(-1f64)).max(0f64);
                    ratio = ratio * (m.albedo / PI);
                    ratio = ratio * light.get_intensity();
                    rendered_color = &rendered_color + &(ratio * &(&light.get_color() * &m.color));
                }
            }
        }
        rendered_color
    }
}
