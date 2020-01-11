mod camera;
mod color;
mod engine;
mod geometry;
mod intersectable;
mod light;
mod material;
mod parser;
mod renderer;
mod texture;

use camera::*;
use color::*;
use engine::*;
use geometry::*;
use light::*;
use material::*;
use renderer::*;
use std::sync::Arc;
use texture::*;

use std::f64::consts::PI;

fn main() -> Result<(), String> {
    let mut scene = get_mesh();

    let mode = 0i8;
    match mode {
        0 => render_scene_console(&mut scene)?,
        1 => render_scene_file(&mut scene)?,
        _ => render_scene_sdl2(&mut scene)?,
    }

    Ok(())
}

fn get_mesh() -> Scene {
    let mut objects: Vec<SceneObject> = vec![];
    let mesh = Arc::new(parser::parse_obj("res/suzanne.obj".to_string()));
    let mut id: usize = 0;
    for triangle in geometry::get_triangles(mesh) {
        objects.push(SceneObject {
            id: id,
            geometry: Object::MeshTriangle(triangle),
            material: Material {
                ambient_color: Coloration::Color(Color {
                    red: 1f64,
                    green: 0f64,
                    blue: 0f64,
                }),
                ambient_reflection: 0.3f64,
                diffuse_color: Coloration::Color(Color {
                    red: 0f64,
                    green: 0f64,
                    blue: 1f64,
                }),
                diffuse_reflection: 0.7f64,
                specular_color: Coloration::Color(BLACK),
                specular_reflection: 0f64,
                shininess: 0f64,
                reflectivity: 0f64,
                transparency: 0f64,
                index_of_refraction: 0f64,
            },
        });
        id += 1;
    }

    let mut lights: Vec<Light> = vec![];
    lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: 0.5f64,
            y: -1f64,
            z: -0.3f64,
        }
        .normalize(),
        intensity: 1f64,
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
    }));

    let ambient_light = AmbientLight {
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
        intensity: 1f64,
    };

    let standard_camera = Camera {
        position: Point3 {
            x: 1f64,
            y: -0.05f64,
            z: 5f64,
        },
        direction: Vector3 {
            x: -0.2f64,
            y: 0f64,
            z: -1f64,
        }
        .normalize(),
        up_direction: Vector3 {
            x: 0f64,
            y: 1f64,
            z: 0f64,
        },
        field_of_view: PI / 4f64,
        x_resolution: 48u16,
        y_resolution: 26u16,
    };

    Scene {
        objects: objects,
        lights: lights,
        ambient_light: ambient_light,
        camera: standard_camera,
    }
}

#[allow(dead_code)]
fn get_transparent_sphere_in_sphere() -> Scene {
    let mut objects: Vec<SceneObject> = vec![];
    objects.push(SceneObject {
        id: 0,
        geometry: Object::Sphere(Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        }),
        material: Material {
            ambient_color: Coloration::Color(BLACK),
            ambient_reflection: 0f64,
            diffuse_color: Coloration::Color(BLACK),
            diffuse_reflection: 0f64,
            specular_color: Coloration::Color(BLACK),
            specular_reflection: 0f64,
            shininess: 0f64,
            reflectivity: 0f64,
            transparency: 1f64,
            index_of_refraction: 1.33f64,
        },
    });
    objects.push(SceneObject {
        id: 1,
        geometry: Object::Sphere(Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 2f64,
        }),
        material: Material {
            ambient_color: Coloration::Color(BLACK),
            ambient_reflection: 0f64,
            diffuse_color: Coloration::Color(BLACK),
            diffuse_reflection: 0f64,
            specular_color: Coloration::Color(BLACK),
            specular_reflection: 0f64,
            shininess: 0f64,
            reflectivity: 0f64,
            transparency: 1f64,
            index_of_refraction: 0.95f64,
        },
    });
    objects.push(SceneObject {
        id: 2,
        geometry: Object::Plane(Plane {
            point: Point3 {
                x: 0f64,
                y: 0f64,
                z: 10f64,
            },
            normal: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            }
            .normalize(),
        }),
        material: Material {
            ambient_color: Coloration::Texture(Texture {
                pixels: get_checkboard(),
                scale: 5f64,
                offset: POINT2_ORIGIN,
            }),
            ambient_reflection: 1f64,
            diffuse_color: Coloration::Color(BLACK),
            diffuse_reflection: 0f64,
            specular_color: Coloration::Color(BLACK),
            specular_reflection: 0f64,
            shininess: 0f64,
            reflectivity: 0f64,
            transparency: 1f64,
            index_of_refraction: 1.33f64,
        },
    });
    let lights: Vec<Light> = vec![];

    let ambient_light = AmbientLight {
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
        intensity: 1f64,
    };

    let standard_camera = Camera {
        position: Point3 {
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
        field_of_view: PI / 5f64,
        x_resolution: 300u16,
        y_resolution: 300u16,
    };

    Scene {
        objects: objects,
        lights: lights,
        ambient_light: ambient_light,
        camera: standard_camera,
    }
}

#[allow(dead_code)]
fn get_spheres_with_plane() -> Scene {
    let mut objects: Vec<SceneObject> = vec![];
    objects.push(SceneObject {
        id: 0,
        geometry: Object::Sphere(Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 5f64,
        }),
        material: Material {
            ambient_color: Coloration::Color(Color {
                red: 0.1f64,
                green: 1f64,
                blue: 0.1f64,
            }),
            ambient_reflection: 1f64,
            diffuse_color: Coloration::Color(Color {
                red: 0.1f64,
                green: 0.8f64,
                blue: 0.1f64,
            }),
            diffuse_reflection: 0.5f64,
            specular_color: Coloration::Color(Color {
                red: 1f64,
                green: 1f64,
                blue: 1f64,
            }),
            specular_reflection: 0.4f64,
            shininess: 40f64,
            reflectivity: 0.3f64,
            transparency: 0f64,
            index_of_refraction: 0f64,
        },
    });
    objects.push(SceneObject {
        id: 1,
        geometry: Object::Plane(Plane {
            point: Point3 {
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
            ambient_color: Coloration::Color(Color {
                red: 0.8f64,
                green: 1f64,
                blue: 0.8f64,
            }),
            ambient_reflection: 0.1f64,
            diffuse_color: Coloration::Color(Color {
                red: 1f64,
                green: 1f64,
                blue: 1f64,
            }),
            diffuse_reflection: 0.4f64,
            specular_color: Coloration::Color(Color {
                red: 0f64,
                green: 0f64,
                blue: 0f64,
            }),
            specular_reflection: 0f64,
            shininess: 1f64,
            reflectivity: 0.2f64,
            transparency: 0f64,
            index_of_refraction: 0f64,
        },
    });
    objects.push(SceneObject {
        id: 2,
        geometry: Object::Sphere(Sphere {
            center: Point3 {
                x: -5f64,
                y: 0f64,
                z: -2f64,
            },
            radius: 2f64,
        }),
        material: Material {
            ambient_color: Coloration::Color(BLACK),
            ambient_reflection: 0f64,
            diffuse_color: Coloration::Color(Color {
                red: 1f64,
                green: 0.1f64,
                blue: 0.1f64,
            }),
            diffuse_reflection: 0.5f64 / PI,
            specular_color: Coloration::Color(BLACK),
            specular_reflection: 0f64,
            shininess: 1f64,
            reflectivity: 0f64,
            transparency: 0.9f64,
            index_of_refraction: 1.33f64,
        },
    });
    let mut lights: Vec<Light> = vec![];
    lights.push(Light::Point3Light(Point3Light {
        origin: Point3 {
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
        position: Point3 {
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
        x_resolution: 500u16,
        y_resolution: 250u16,
    };

    Scene {
        objects: objects,
        lights: lights,
        ambient_light: ambient_light,
        camera: standard_camera,
    }
}
