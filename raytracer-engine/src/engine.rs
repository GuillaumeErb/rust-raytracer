use crate::camera::*;
use crate::color::*;
use crate::geometry::*;
use crate::intersectable::*;
use crate::kdtree::build_kd_tree;
use crate::kdtree::KDTree;
use crate::light::*;
use crate::material::*;
use rayon::prelude::*;
use serde::ser::SerializeStruct;
use serde::{de, ser, Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

const MAX_BOUNCES: u8 = 4;

pub type SceneObjectId = usize;

pub struct SceneObject {
    pub id: SceneObjectId,
    pub geometry: Object,
    pub material: Material,
}

pub struct SceneObjects {
    pub objects: Vec<SceneObject>,
    pub kd_tree: Option<KDTree>,
}

impl SceneObjects {
    pub fn initialize(objects: Vec<SceneObject>) -> Self {
        SceneObjects {
            objects: objects,
            kd_tree: None,
        }
    }

    pub fn build_kd_tree(&mut self) {
        self.kd_tree = Some(build_kd_tree(&self.objects));
    }
}

pub struct Scene {
    pub objects: SceneObjects,
    pub ambient_light: AmbientLight,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

pub struct SceneIntersection<'a> {
    pub intersection: Intersection,
    pub object: &'a SceneObject,
}

impl SceneIntersection<'_> {
    pub fn get_normal(&self, point: &Point3) -> Vector3 {
        match self.intersection.triangle_u_v {
            Some(uv) => {
                // kind of clumsy, but I don't want to recompute uv ...
                match &self.object.geometry {
                    Object::MeshTriangle(triangle) => get_triangle_normal(triangle, &uv, point),
                    _ => self.object.geometry.get_normal(point),
                }
            }
            None => self.object.geometry.get_normal(point),
        }
    }
}

pub struct TracedRay {
    pub ray: Ray,
    pub inside_objects: Vec<SceneObjectId>,
}

pub fn render(scene: &Scene) -> HashMap<(u16, u16), Color> {
    let now = Instant::now();
    let viewport = scene.camera.generate_viewport();
    let screen: HashMap<_, _> = viewport
        //.iter()
        .par_iter()
        .map(|view_ray| {
            let coordinates = (view_ray.x, view_ray.y);
            let result = render_pixel(scene, view_ray.ray.clone());
            (coordinates, result)
        })
        .collect();
    println!("{}", now.elapsed().as_millis());
    screen
}

pub fn render_pixel(scene: &Scene, ray: Ray) -> Color {
    let mut traced_ray = TracedRay {
        ray: ray,
        inside_objects: vec![],
    };
    cast_ray(&scene, &mut traced_ray, MAX_BOUNCES)
}

pub fn is_in_shadow(point: &Point3, light: &Light, scene: &Scene) -> bool {
    let light_direction = light.get_direction(point);
    let shadow_ray = Ray {
        origin: *point,
        direction: light_direction.times(-1f64),
    };

    scene
        .objects
        .objects
        .iter()
        .filter_map(|object| object.geometry.intersect(&shadow_ray))
        .any(|_d| true)
}

pub fn cast_ray(scene: &Scene, ray: &TracedRay, max_bounces: u8) -> Color {
    get_closest_intersection(scene, ray)
        .map(|i| {
            (*i.object)
                .material
                .render_color(ray, &i, &scene, max_bounces)
        })
        .unwrap_or(BLACK)
}

fn get_closest_intersection<'a>(
    scene: &'a Scene,
    ray: &TracedRay,
) -> Option<SceneIntersection<'a>> {
    let scene_objects = &scene.objects;
    let candidates: Vec<&SceneObject> = match &scene_objects.kd_tree {
        Some(kd_tree) => kd_tree
            .get_leafs_intersecting(&ray.ray)
            .union(&kd_tree.tree.objects) // adding unbound objects
            .map(|&index| &scene_objects.objects[index])
            .collect(),
        None => scene_objects.objects.iter().map(|object| object).collect(),
    };

    let result = candidates
        .iter()
        .filter_map(|object| {
            object
                .geometry
                .intersect(&ray.ray)
                .map(|intersection| SceneIntersection {
                    intersection: intersection,
                    object: &object,
                })
        })
        .min_by(|i1, i2| {
            i1.intersection
                .distance
                .partial_cmp(&i2.intersection.distance)
                .unwrap()
        });
    result
}

pub fn get_object<'a>(scene: &'a Scene, x: u16, y: u16) -> Option<usize> {
    let ray = scene.camera.get_ray(x, y);
    let intersection = get_closest_intersection(
        scene,
        &TracedRay {
            ray: ray,
            inside_objects: vec![],
        },
    );
    match intersection {
        Some(x) => Some(x.object.id),
        None => None,
    }
}
