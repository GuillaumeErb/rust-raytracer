mod camera;
mod color;
mod geometry;
mod intersectable;
mod light;
mod material;
mod renderer;

use camera::Camera;
use camera::GeneratingViewRays;
use camera::StandardCamera;
use color::Color;
use color::BLACK;
use geometry::Object;
use geometry::Plane;
use geometry::Point;
use geometry::Ray;
use geometry::Sphere;
use geometry::Vector3;
use intersectable::Intersectable;
use light::AmbientLight;
use light::DirectionalLight;
use light::Light;
use light::PointLight;
use material::LambertMaterial;
use material::Material;
use material::PhongMaterial;
use rayon::prelude::*;
use renderer::render_scene_sdl2;

use std::f64::consts::PI;

const MAX_BOUNCES: u8 = 1;

fn main() -> Result<(), String> {
    let mut objects: Vec<ObjectWithMaterial> = vec![];
    objects.push(ObjectWithMaterial {
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 5f64,
        }),
        material: Material::PhongMaterial(PhongMaterial {
            ambient_color: Color {
                red: 0.1f64,
                green: 1f64,
                blue: 0.1f64,
            },
            ambient_reflection: 1f64,
            diffuse_color: Color {
                red: 0.1f64,
                green: 0.8f64,
                blue: 0.1f64,
            },
            diffuse_reflection: 0.5f64,
            specular_color: Color {
                red: 1f64,
                green: 1f64,
                blue: 1f64,
            },
            specular_reflection: 0.4f64,
            shininess: 40f64,
            reflectivity: 0.3f64,
        }),
    });
    objects.push(ObjectWithMaterial {
        geometry: Object::Plane(Plane {
            point: Point {
                x: 5f64,
                y: 0f64,
                z: 0f64,
            },
            normal: Vector3 {
                x: -1f64,
                y: 0f64,
                z: -0.5f64,
            }
            .normalize(),
        }),
        material: Material::PhongMaterial(PhongMaterial {
            ambient_color: Color {
                red: 0.8f64,
                green: 1f64,
                blue: 0.8f64,
            },
            ambient_reflection: 0.1f64,
            diffuse_color: Color {
                red: 1f64,
                green: 1f64,
                blue: 1f64,
            },
            diffuse_reflection: 0.4f64,
            specular_color: Color {
                red: 0f64,
                green: 0f64,
                blue: 0f64,
            },
            specular_reflection: 0f64,
            shininess: 1f64,
            reflectivity: 0.2f64,
        }),
    });
    objects.push(ObjectWithMaterial {
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: -5f64,
                y: 0f64,
                z: -2f64,
            },
            radius: 2f64,
        }),
        material: Material::LambertMaterial(LambertMaterial {
            color: Color {
                red: 1f64,
                green: 0.1f64,
                blue: 0.1f64,
            },
            albedo: 1f64,
        }),
    });

    let mut lights: Vec<Light> = vec![];
    lights.push(Light::PointLight(PointLight {
        origin: Point {
            x: 100f64,
            y: 0f64,
            z: 0f64,
        },
        intensity: 1f64,
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
    }));

    lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: 4f64,
            y: 2f64,
            z: 1f64,
        }
        .normalize(),
        intensity: 2f64,
        color: Color {
            red: 0.2f64,
            green: 0.5f64,
            blue: 1f64,
        },
    }));

    let ambient_light = AmbientLight {
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
        intensity: 0.1f64,
    };

    let standard_camera = Camera::StandardCamera(StandardCamera {
        position: Point {
            x: 0f64,
            y: 0f64,
            z: -20f64,
        },
        direction: Vector3 {
            x: 0f64,
            y: 0f64,
            z: 1f64,
        },
        up_direction: Vector3 {
            x: 0f64,
            y: 1f64,
            z: 0f64,
        },
        field_of_view: PI / 2f64,
        x_resolution: 600u16,
        y_resolution: 400u16,
    });

    let mut scene = Scene {
        objects: objects,
        lights: lights,
        ambient_light: ambient_light,
        camera: standard_camera,
    };

    render_scene_sdl2(&mut scene)?;

    Ok(())
}

pub fn is_in_shadow(point: &Point, light: &Light, scene: &Scene) -> bool {
    let light_direction = light.get_direction(point);
    let shadow_ray = Ray {
        origin: *point,
        direction: light_direction.times(-1f64),
    };

    scene
        .objects
        .par_iter()
        .filter_map(|object| object.geometry.intersect(&shadow_ray))
        .any(|_d| true)
}

struct ObjectWithMaterial {
    geometry: Object,
    material: Material,
}

pub struct Scene {
    objects: Vec<ObjectWithMaterial>,
    ambient_light: AmbientLight,
    lights: Vec<Light>,
    camera: Camera,
}

pub struct Intersection<'a> {
    distance: f64,
    object: &'a ObjectWithMaterial,
}

pub fn cast_ray(scene: &Scene, ray: &Ray, max_bounces: u8) -> Color {
    let intersection = scene
        .objects
        .par_iter()
        .filter_map(|object| {
            object.geometry.intersect(ray).map(|distance| Intersection {
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
