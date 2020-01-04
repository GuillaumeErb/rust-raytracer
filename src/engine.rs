use crate::camera::{Camera, GeneratingViewRays};
use crate::color::{Color, BLACK};
use crate::geometry::{Object, Point, Ray};
use crate::intersectable::Intersectable;
use crate::light::{AmbientLight, Light};
use crate::material::Material;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

const MAX_BOUNCES: u8 = 4;

pub type SceneObjectId = usize;

pub struct SceneObject {
    pub id: SceneObjectId,
    pub geometry: Object,
    pub material: Material,
}

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub ambient_light: AmbientLight,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a SceneObject,
}

pub struct TracedRay {
    pub ray: Ray,
    pub inside_objects: Vec<SceneObjectId>,
}

pub fn render(scene: &Scene) -> HashMap<(u16, u16), Color> {
    let now = Instant::now();
    let viewport = scene.camera.generate_viewport();
    let screen: HashMap<_, _> = viewport
        .par_iter()
        .map(|view_ray| {
            let mut traced_ray = TracedRay {
                ray: view_ray.ray.clone(),
                inside_objects: vec![],
            };
            let coordinates = (view_ray.x, view_ray.y);
            let result = cast_ray(&scene, &mut traced_ray, MAX_BOUNCES);
            (coordinates, result)
        })
        .collect();
    println!("{}", now.elapsed().as_millis());
    screen
}

pub fn is_in_shadow(point: &Point, light: &Light, scene: &Scene) -> bool {
    let light_direction = light.get_direction(point);
    let shadow_ray = Ray {
        origin: *point,
        direction: light_direction.times(-1f64),
    };

    scene
        .objects
        .iter()
        .filter_map(|object| object.geometry.intersect(&shadow_ray))
        .any(|_d| true)
}

pub fn cast_ray(scene: &Scene, ray: &TracedRay, max_bounces: u8) -> Color {
    let intersection = scene
        .objects
        .iter()
        .filter_map(|object| {
            object
                .geometry
                .intersect(&ray.ray)
                .map(|distance| Intersection {
                    distance: distance,
                    object: &object,
                })
        })
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());

    intersection
        .map(|i| {
            (*i.object)
                .material
                .render_color(ray, &i, &scene, max_bounces)
        })
        .unwrap_or(BLACK)
}
