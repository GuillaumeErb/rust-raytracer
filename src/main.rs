mod camera;
mod color;
mod engine;
mod geometry;
mod intersectable;
mod light;
mod material;
mod renderer;

use camera::Camera;
use camera::ViewRay;
use color::Color;
use color::BLACK;
use engine::Scene;
use engine::SceneObject;
use geometry::Object;
use geometry::Plane;
use geometry::Point;
use geometry::Ray;
use geometry::Sphere;
use geometry::Vector3;
use light::AmbientLight;
use light::DirectionalLight;
use light::Light;
use light::PointLight;
use material::Material;
use renderer::render_scene_sdl2;

use std::f64::consts::PI;

const MAX_BOUNCES: u8 = 1;

fn main() -> Result<(), String> {
    let mut objects: Vec<SceneObject> = vec![];
    objects.push(SceneObject {
        id: 0,
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 5f64,
        }),
        material: Material {
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
        },
    });
    objects.push(SceneObject {
        id: 1,
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
        material: Material {
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
        },
    });
    objects.push(SceneObject {
        id: 2,
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: -5f64,
                y: 0f64,
                z: -2f64,
            },
            radius: 2f64,
        }),
        material: Material {
            ambient_color: BLACK,
            ambient_reflection: 0f64,
            diffuse_color: Color {
                red: 1f64,
                green: 0.1f64,
                blue: 0.1f64,
            },
            diffuse_reflection: 1f64 / PI,
            specular_color: BLACK,
            specular_reflection: 0f64,
            shininess: 1f64,
            reflectivity: 0f64,
        },
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

    let standard_camera = Camera {
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
    };

    let mut scene = Scene {
        objects: objects,
        lights: lights,
        ambient_light: ambient_light,
        camera: standard_camera,
    };

    render_scene_sdl2(&mut scene)?;

    Ok(())
}
