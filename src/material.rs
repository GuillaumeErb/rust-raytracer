use crate::cast_ray;
use crate::is_in_shadow;
use crate::Color;
use crate::Intersection;
use crate::Ray;
use crate::Scene;
use crate::PI;

#[derive(Debug)]
pub enum Material {
    ConstantMaterial(ConstantMaterial),
    LambertMaterial(LambertMaterial),
    PhongMaterial(PhongMaterial),
}

#[derive(Debug)]
pub struct ConstantMaterial {
    pub color: Color,
}

#[derive(Debug)]
pub struct LambertMaterial {
    pub color: Color,
    pub albedo: f64, // between 0 and 1
}

#[derive(Debug)]
pub struct PhongMaterial {
    pub ambient_color: Color,
    pub ambient_reflection: f64,
    pub diffuse_color: Color,
    pub diffuse_reflection: f64,
    pub specular_color: Color,
    pub specular_reflection: f64,
    pub shininess: f64,
    pub reflectivity: f64,
}

impl Material {
    pub fn render_color(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        scene: &Scene,
        max_bounces: u8,
    ) -> Color {
        let point_precise = ray.origin.add(&ray.direction.times(intersection.distance));
        let normal = intersection.object.geometry.get_normal(&point_precise);
        let point = point_precise.add(&normal.times(1e-6));

        let mut rendered_color = &(&scene.ambient_light.color * &self.get_ambient_color())
            * scene.ambient_light.intensity;
        for light in &scene.lights {
            if is_in_shadow(&point, &light, scene) {
                continue;
            }
            match *self {
                Material::ConstantMaterial(ref m) => {
                    rendered_color = &rendered_color + &(&light.get_color() * &m.color)
                }
                Material::LambertMaterial(ref m) => {
                    let mut ratio = normal.dot(&light.get_direction().times(-1f64)).max(0f64);
                    ratio = ratio * (m.albedo / PI);
                    ratio = ratio * light.get_intensity();
                    rendered_color = &rendered_color + &(ratio * &(&light.get_color() * &m.color));
                }
                Material::PhongMaterial(ref m) => {
                    let to_light = &light.get_direction().times(-1f64);
                    let to_eye = ray.direction.times(-1f64);
                    let light_normal_reflection = to_light.symmetry(&normal);
                    let diffuse = m.diffuse_reflection * normal.dot(to_light).max(0f64);
                    let specular = m.specular_reflection
                        * (&light_normal_reflection.dot(&to_eye).max(0f64)).powf(m.shininess);

                    rendered_color = &rendered_color
                        + &(diffuse
                            * &(&(&light.get_color() * light.get_intensity()) * &m.diffuse_color));
                    rendered_color = &rendered_color
                        + &(specular
                            * &(&(&light.get_color() * light.get_intensity()) * &m.specular_color));
                }
            }
        }
        if self.get_reflectivity() > 1e-6 && max_bounces > 0 {
            let reflected_ray = Ray {
                origin: point,
                direction: ray.direction.times(-1f64).symmetry(&normal),
            };
            rendered_color = &rendered_color
                + &(self.get_reflectivity() * &cast_ray(scene, &reflected_ray, max_bounces - 1));
        }
        rendered_color
    }

    pub fn get_ambient_color(&self) -> Color {
        match *self {
            Material::ConstantMaterial(ref m) => m.color,
            Material::LambertMaterial(ref m) => m.color,
            Material::PhongMaterial(ref m) => &m.ambient_color * m.ambient_reflection,
        }
    }

    pub fn get_reflectivity(&self) -> f64 {
        match *self {
            Material::ConstantMaterial(ref _m) => 0f64,
            Material::LambertMaterial(ref _m) => 0f64,
            Material::PhongMaterial(ref m) => m.reflectivity,
        }
    }
}
