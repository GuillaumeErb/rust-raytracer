mod camera;
mod color;
mod geometry;
mod intersectable;
mod light;
mod material;
mod renderer;

use camera::Camera;
use camera::GeneratingViewRays;
use camera::OrthographicCamera;
use camera::StandardCamera;
use color::Color;
use geometry::Object;
use geometry::Plane;
use geometry::Point;
use geometry::Ray;
use geometry::Sphere;
use geometry::Vector3;
use intersectable::Intersectable;
use light::DirectionalLight;
use light::Light;
use material::Material;
use rayon::prelude::*;
use renderer::render_scene_sdl2;

use std::f64::consts::PI;

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
        material: Material::LambertMaterial(LambertMaterial {
            color: Color {
                red: 0f64,
                green: 1f64,
                blue: 0f64,
            },
            albedo: 1f64,
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
                green: 0f64,
                blue: 0f64,
            },
            albedo: 1f64,
        }),
    });
    /*
    objects.push(ObjectWithMaterial {
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: 350f64,
                y: 0f64,
                z: -70f64,
            },
            radius: 50f64,
        }),
        material: Material::LambertMaterial(LambertMaterial {
            color: Color {
                red: 0f64,
                green: 0f64,
                blue: 1f64,
            },
        }),
    });*/
    let mut lights: Vec<Light> = vec![];
    lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: 1f64,
            y: 0f64,
            z: 0f64,
        }
        .normalize(),
        intensity: 2f64,
        color: Color {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
        },
    }));
    /*lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: 0f64,
            y: 0f64,
            z: 1f64,
        }
        .normalize(),
    }));*/

    let orthoCamera = Camera::OrthographicCamera(OrthographicCamera {
        x_resolution: 800u16,
        y_resolution: 1000u16,
    });

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
    /*
        let standard_camera = Camera::StandardCamera(StandardCamera {
            position: Point {
                x: -3f64,
                y: 0f64,
                z: -10f64,
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
            field_of_view: PI / 10f64,
            x_resolution: 2u16,
            y_resolution: 2u16,
        });
    */
    let mut scene = Scene {
        objects: objects,
        lights: lights,
        camera: standard_camera,
    };

    //render_scene(scene);

    render_scene_sdl2(&mut scene)?;

    Ok(())
}

#[derive(Debug)]
pub struct LambertMaterial {
    color: Color,
    albedo: f64, // between 0 and 1
}

impl LambertMaterial {
    pub fn render_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
        let point = ray.origin.add(&ray.direction.times(intersection.distance));
        let normal = intersection.object.geometry.get_normal(&point);
        let mut diffuse_lights = BLACK;
        for light in &scene.lights {
            //println!("Is is shadow ... ?");
            if is_in_shadow(&point, &light, scene) {
                continue;
            }
            diffuse_lights = &diffuse_lights
                + &(normal.dot(&light.get_direction().times(-1f64)).max(0f64) * self.albedo / PI
                    * &light.get_intensity()
                    * &light.get_color());
        }
        &self.color * &(diffuse_lights/*.powi(5)*5f64*/)
    }
}

pub fn is_in_shadow(point: &Point, light: &Light, scene: &Scene) -> bool {
    let light_direction = light.get_direction();
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

pub const BLACK: Color = Color {
    red: 0f64,
    green: 0f64,
    blue: 0f64,
};

pub struct Scene {
    objects: Vec<ObjectWithMaterial>,
    lights: Vec<Light>,
    camera: Camera,
}

pub struct Intersection<'a> {
    distance: f64,
    object: &'a ObjectWithMaterial,
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    let intersection = scene
        .objects
        .iter()
        .filter_map(|object| {
            object.geometry.intersect(ray).map(|distance| Intersection {
                distance: distance,
                object: &object,
            })
        })
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());

    intersection
        .map(|i| (*i.object).material.render_color(ray, &i, &scene))
        .unwrap_or(BLACK)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use material::ConstantMaterial;

    #[test]
    fn single_sphere() {
        let mut objects: Vec<ObjectWithMaterial> = vec![];
        objects.push(ObjectWithMaterial {
            geometry: Object::Sphere(Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
            }),
            material: Material::ConstantMaterial(ConstantMaterial {
                color: Color {
                    red: 0f64,
                    green: 1f64,
                    blue: 0f64,
                },
            }),
        });
        let scene = Scene {
            objects: objects,
            lights: vec![],
            camera: Camera::OrthographicCamera(OrthographicCamera {
                x_resolution: 25u16,
                y_resolution: 50u16,
            }),
        };
        let ray = Ray {
            origin: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            },
        };
        let resulting_color = cast_ray(&scene, &ray);
        assert_eq!(
            resulting_color,
            Color {
                red: 0f64,
                green: 1f64,
                blue: 0f64
            }
        );
    }

    #[test]
    fn single_sphere_lambert() {
        let mut objects: Vec<ObjectWithMaterial> = vec![];
        objects.push(ObjectWithMaterial {
            geometry: Object::Sphere(Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
            }),
            material: Material::LambertMaterial(LambertMaterial {
                color: Color {
                    red: 0f64,
                    green: 1f64,
                    blue: 0f64,
                },
                albedo: 1f64,
            }),
        });
        let mut lights: Vec<Light> = vec![];
        lights.push(Light::DirectionalLight(DirectionalLight {
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            }
            .normalize(),
            intensity: 1f64,
            color: Color {
                red: 1f64,
                green: 1f64,
                blue: 1f64,
            },
        }));
        let scene = Scene {
            objects: objects,
            lights: lights,
            camera: Camera::OrthographicCamera(OrthographicCamera {
                x_resolution: 25u16,
                y_resolution: 50u16,
            }),
        };
        let ray = Ray {
            origin: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            },
        };
        let resulting_color = cast_ray(&scene, &ray);
        assert_eq!(
            resulting_color,
            Color {
                red: 0f64,
                green: 1f64,
                blue: 0f64
            }
        );
    }

    #[test]
    fn two_aligned_spheres() {
        let mut objects: Vec<ObjectWithMaterial> = vec![];
        objects.push(ObjectWithMaterial {
            geometry: Object::Sphere(Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
            }),
            material: Material::ConstantMaterial(ConstantMaterial {
                color: Color {
                    red: 0f64,
                    green: 1f64,
                    blue: 0f64,
                },
            }),
        });
        objects.push(ObjectWithMaterial {
            geometry: Object::Sphere(Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -45f64,
                },
                radius: 6f64,
            }),
            material: Material::ConstantMaterial(ConstantMaterial {
                color: Color {
                    red: 1f64,
                    green: 0f64,
                    blue: 0f64,
                },
            }),
        });

        let scene = Scene {
            objects: objects,
            lights: vec![],
            camera: Camera::OrthographicCamera(OrthographicCamera {
                x_resolution: 25u16,
                y_resolution: 50u16,
            }),
        };
        let ray = Ray {
            origin: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            },
        };
        let resulting_color = cast_ray(&scene, &ray);
        assert_eq!(
            resulting_color,
            Color {
                red: 1f64,
                green: 0f64,
                blue: 0f64
            }
        );
    }

    #[test]
    fn plane_test() {
        let mut objects: Vec<ObjectWithMaterial> = vec![];
        objects.push(ObjectWithMaterial {
            geometry: Object::Plane(Plane {
                point: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                normal: Vector3 {
                    x: 0f64,
                    y: 0f64,
                    z: 1f64,
                },
            }),
            material: Material::ConstantMaterial(ConstantMaterial {
                color: Color {
                    red: 0f64,
                    green: 0f64,
                    blue: 1f64,
                },
            }),
        });
        let scene = Scene {
            objects: objects,
            lights: vec![],
            camera: Camera::OrthographicCamera(OrthographicCamera {
                x_resolution: 25u16,
                y_resolution: 50u16,
            }),
        };
        let ray = Ray {
            origin: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: -1f64,
            },
        };
        let resulting_color = cast_ray(&scene, &ray);
        assert_eq!(
            resulting_color,
            Color {
                red: 0f64,
                green: 0f64,
                blue: 1f64
            }
        );
    }
}
