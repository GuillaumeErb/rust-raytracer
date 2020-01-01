use crate::cast_ray;
use crate::is_in_shadow;
use crate::Color;
use crate::Intersection;
use crate::Ray;
use crate::Scene;

#[derive(Debug)]
pub struct Material {
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

        let mut rendered_color = &(&scene.ambient_light.color
            * &(&self.ambient_color * self.ambient_reflection))
            * scene.ambient_light.intensity;
        for light in &scene.lights {
            if is_in_shadow(&point, &light, scene) {
                continue;
            }

            let to_light = &light.get_direction(&point).times(-1f64);
            let to_eye = ray.direction.times(-1f64);
            let light_normal_reflection = to_light.symmetry(&normal);
            let diffuse = self.diffuse_reflection * normal.dot(to_light).max(0f64);
            let specular = self.specular_reflection
                * (&light_normal_reflection.dot(&to_eye).max(0f64)).powf(self.shininess);

            rendered_color = &rendered_color
                + &(diffuse
                    * &(&(&light.get_color() * light.get_intensity()) * &self.diffuse_color));
            rendered_color = &rendered_color
                + &(specular
                    * &(&(&light.get_color() * light.get_intensity()) * &self.specular_color));
        }
        if self.reflectivity > 1e-6 && max_bounces > 0 {
            let reflected_ray = Ray {
                origin: point,
                direction: ray.direction.times(-1f64).symmetry(&normal),
            };
            rendered_color = &rendered_color
                + &(self.reflectivity * &cast_ray(scene, &reflected_ray, max_bounces - 1));
        }
        rendered_color
    }
}
