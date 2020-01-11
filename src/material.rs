use crate::color::{Color, BLACK};
use crate::engine::{cast_ray, is_in_shadow, Scene, SceneIntersection, TracedRay};
use crate::geometry::{Object, Point3, Ray, Vector3};
use crate::texture::Texture;
use std::mem::swap;

#[derive(Debug)]
pub struct Material {
    pub ambient_color: Coloration,
    pub ambient_reflection: f64,
    pub diffuse_color: Coloration,
    pub diffuse_reflection: f64,
    pub specular_color: Coloration,
    pub specular_reflection: f64,
    pub shininess: f64,
    pub reflectivity: f64,
    pub transparency: f64,
    pub index_of_refraction: f64,
}

#[derive(Debug)]
pub enum Coloration {
    Color(Color),
    Texture(Texture),
}

impl Coloration {
    pub fn color(&self, object: &Object, point: &Point3) -> Color {
        match self {
            Coloration::Color(c) => c.clone(),
            Coloration::Texture(t) => t.get_color(object, point),
        }
    }
}

impl Material {
    pub fn render_color(
        &self,
        ray: &TracedRay,
        intersection: &SceneIntersection,
        scene: &Scene,
        max_bounces: u8,
    ) -> Color {
        let point_precise = ray
            .ray
            .origin
            .add(&ray.ray.direction.times(intersection.intersection.distance));
        let normal = intersection.get_normal(&point_precise);
        let point = point_precise.add(&normal.times(1e-6));

        let mut rendered_color = &(&scene.ambient_light.color
            * &(&self
                .ambient_color
                .color(&intersection.object.geometry, &point_precise)
                * self.ambient_reflection))
            * scene.ambient_light.intensity;
        for light in &scene.lights {
            if is_in_shadow(&point, &light, scene) {
                continue;
            }

            let to_light = &light.get_direction(&point).times(-1f64);
            let to_eye = ray.ray.direction.times(-1f64);
            let light_normal_reflection = to_light.symmetry(&normal);
            let diffuse = self.diffuse_reflection * normal.dot(to_light).max(0f64);

            let specular = self.specular_reflection
                * (&light_normal_reflection.dot(&to_eye).max(0f64)).powf(self.shininess);

            rendered_color = &rendered_color
                + &(diffuse
                    * &(&(&light.get_color() * light.get_intensity())
                        * &self
                            .diffuse_color
                            .color(&intersection.object.geometry, &point_precise)));
            rendered_color = &rendered_color
                + &(specular
                    * &(&(&light.get_color() * light.get_intensity())
                        * &self
                            .specular_color
                            .color(&intersection.object.geometry, &point_precise)));
        }

        if self.reflectivity > 1e-6 && max_bounces > 0 {
            let reflected_ray = Ray {
                origin: point,
                direction: ray.ray.direction.times(-1f64).symmetry(&normal),
            };
            let reflected_traced_ray = TracedRay {
                ray: reflected_ray,
                inside_objects: ray.inside_objects.clone(),
            };
            let reflected_color = cast_ray(scene, &reflected_traced_ray, max_bounces - 1);
            rendered_color = &rendered_color + &(self.reflectivity * &reflected_color);
        }
        if self.transparency > 1e-6 && max_bounces > 0 {
            let outside_index_of_refraction = match ray.inside_objects.last() {
                Some(x) => scene.objects[*x].material.index_of_refraction,
                None => 1f64,
            };

            let kr = fresnel(
                &ray.ray.direction,
                &normal,
                self.index_of_refraction,
                outside_index_of_refraction,
            );
            let outside = ray.ray.direction.dot(&normal) < 0f64;

            let mut refracted_color = BLACK;
            if kr < 1f64 {
                let refracted_ray_origin;
                let mut new_inside_objects = ray.inside_objects.clone();
                if outside {
                    refracted_ray_origin = point_precise.add(&normal.times(-1e-6));
                    new_inside_objects.push(intersection.object.id);
                } else {
                    refracted_ray_origin = point_precise.add(&normal.times(1e-6));
                    new_inside_objects.retain(|&x| x != intersection.object.id);
                };
                let refracted_direction = refract(
                    &ray.ray.direction,
                    &normal,
                    self.index_of_refraction,
                    outside_index_of_refraction,
                );
                let refracted_ray = Ray {
                    origin: refracted_ray_origin,
                    direction: refracted_direction,
                };
                let refracted_traced_ray = TracedRay {
                    ray: refracted_ray,
                    inside_objects: new_inside_objects,
                };
                refracted_color = cast_ray(scene, &refracted_traced_ray, max_bounces - 1);
            }

            let reflected_direction = ray.ray.direction.times(-1f64).symmetry(&normal);
            let reflected_ray_origin = if outside {
                point_precise.add(&normal.times(1e-6))
            } else {
                point_precise.add(&normal.times(-1e-6))
            };

            let reflected_ray = Ray {
                origin: reflected_ray_origin,
                direction: reflected_direction,
            };
            let reflected_traced_ray = TracedRay {
                ray: reflected_ray,
                inside_objects: ray.inside_objects.clone(),
            };
            let reflected_color = cast_ray(scene, &reflected_traced_ray, max_bounces - 1);

            rendered_color = &rendered_color
                + &(self.transparency
                    * &(&(&reflected_color * kr) + &(&refracted_color * (1f64 - kr))));
        }
        rendered_color
    }
}

fn fresnel(
    incident: &Vector3,
    normal: &Vector3,
    index_of_refraction: f64,
    outside_index_of_refraction: f64,
) -> f64 {
    let mut cosi = incident.dot(normal);
    let mut etai = outside_index_of_refraction;
    let mut etat = index_of_refraction;
    if cosi > 0f64 {
        swap(&mut etai, &mut etat);
    }

    let sint = etai / etat * (1f64 - cosi * cosi).max(0f64).sqrt();

    if sint >= 1f64 {
        1f64
    } else {
        let cost = (1f64 - sint * sint).max(0f64).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2f64
    }
}

fn refract(
    incident: &Vector3,
    normal: &Vector3,
    index_of_refraction: f64,
    outside_index_of_refraction: f64,
) -> Vector3 {
    let mut cosi = incident.dot(normal).min(1f64).max(-1f64);
    let mut etai = outside_index_of_refraction;
    let mut etat = index_of_refraction;
    let n;
    if cosi < 0f64 {
        cosi = -cosi;
        n = *normal;
    } else {
        swap(&mut etai, &mut etat);
        n = normal.times(-1f64);
    }

    let eta = etai / etat;
    let k = 1f64 - eta * eta * (1f64 - cosi * cosi);
    if k < 0f64 {
        Vector3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    } else {
        (incident.times(eta))
            .plus(&n.times(eta * cosi - k.sqrt()))
            .normalize()
    }
}
