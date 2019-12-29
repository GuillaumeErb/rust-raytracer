mod camera;
mod geometry;
mod intersectable;

use camera::GeneratingViewRays;
use camera::OrthographicCamera;
use geometry::Object;
use geometry::Plane;
use geometry::Point;
use geometry::Ray;
use geometry::Sphere;
use geometry::Vector3;
use intersectable::Intersectable;
use rayon::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut objects: Vec<ObjectWithMaterial> = vec![];
    objects.push(ObjectWithMaterial {
        geometry: Object::Sphere(Sphere {
            center: Point {
                x: 0f64,
                y: 0f64,
                z: -70f64,
            },
            radius: 200f64,
        }),
        material: Material::LambertMaterial(LambertMaterial {
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
                x: 40f64,
                y: 20f64,
                z: -10f64,
            },
            radius: 150f64,
        }),
        material: Material::LambertMaterial(LambertMaterial {
            color: Color {
                red: 1f64,
                green: 0f64,
                blue: 0f64,
            },
        }),
    });
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
    });
    let mut lights: Vec<Light> = vec![];
    lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: -1f64,
            y: 0f64,
            z: 0f64,
        }
        .normalize(),
    }));
    /*lights.push(Light::DirectionalLight(DirectionalLight {
        direction: Vector3 {
            x: 0f64,
            y: 0f64,
            z: 1f64,
        }
        .normalize(),
    }));*/
    let mut scene = Scene {
        objects: objects,
        lights: lights,
        camera: OrthographicCamera {
            x_resolution: 1000u16,
            y_resolution: 800u16,
        },
    };

    //render_scene(scene);

    render_scene_sdl2(&mut scene)?;

    Ok(())
}

#[derive(Debug)]
pub enum Material {
    ConstantMaterial(ConstantMaterial),
    LambertMaterial(LambertMaterial),
}

impl Material {
    pub fn render_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
        match *self {
            Material::ConstantMaterial(ref m) => m.color,
            Material::LambertMaterial(ref m) => m.render_color(ray, intersection, scene),
        }
    }
}

#[derive(Debug)]
pub struct ConstantMaterial {
    color: Color,
}

#[derive(Debug)]
pub struct LambertMaterial {
    color: Color,
}

impl LambertMaterial {
    pub fn render_color(&self, ray: &Ray, intersection: &Intersection, scene: &Scene) -> Color {
        let point = ray.origin.add(&ray.direction.times(intersection.distance));
        let normal = intersection.object.geometry.get_normal(&point);
        let mut diffuse_lights = 0f64;
        for light in &scene.lights {
            if is_in_shadow(&point, &light, scene) {
                continue;
            }
            diffuse_lights += normal.dot(&light.get_direction().times(-1f64)).max(0f64);
        }
        self.color.times(diffuse_lights /*.powi(5)*5f64*/)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn times(&self, scalar: f64) -> Color {
        Color {
            red: (self.red * scalar).min(1f64).max(0f64),
            green: (self.green * scalar).min(1f64).max(0f64),
            blue: (self.blue * scalar).min(1f64).max(0f64),
        }
    }
}

impl From<Color> for ansi_term::Color {
    fn from(item: Color) -> Self {
        ansi_term::Color::RGB(
            (item.red * 255f64).round() as u8,
            (item.green * 255f64).round() as u8,
            (item.blue * 255f64).round() as u8,
        )
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(item: Color) -> Self {
        image::Rgb([
            ((item.red as f32) * 255.0) as u8,
            ((item.green as f32) * 255.0) as u8,
            ((item.blue as f32) * 255.0) as u8,
        ])
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(item: Color) -> Self {
        sdl2::pixels::Color::RGB(
            ((item.red as f32) * 255.0) as u8,
            ((item.green as f32) * 255.0) as u8,
            ((item.blue as f32) * 255.0) as u8,
        )
    }
}

#[derive(Debug)]
pub enum Light {
    DirectionalLight(DirectionalLight),
}

impl Light {
    pub fn get_direction(&self) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref light) => light.direction,
        }
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
        .iter()
        .filter_map(|object| object.geometry.intersect(&shadow_ray))
        .any(|_d| true)
}

#[derive(Debug)]
pub struct DirectionalLight {
    direction: Vector3,
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
    camera: OrthographicCamera,
}

pub fn render_scene_console(scene: Scene) {
    let viewport = scene.camera.generate_viewport();
    let screen: HashMap<_, _> = viewport
        .into_iter()
        .map(|view_ray| ((view_ray.x, view_ray.y), cast_ray(&scene, &view_ray.ray)))
        .collect();

    for y in 0..scene.camera.y_resolution {
        for x in 0..scene.camera.x_resolution {
            let ansi_color: ansi_term::Color = screen[&(x, y)].into();
            print!("{}", ansi_color.paint("█"));
        }
        println!();
    }
}

pub fn render_scene(scene: Scene) {
    let viewport = scene.camera.generate_viewport();
    let screen: HashMap<_, _> = viewport
        .into_iter()
        .map(|view_ray| ((view_ray.x, view_ray.y), cast_ray(&scene, &view_ray.ray)))
        .collect();

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(
        scene.camera.x_resolution as u32,
        scene.camera.y_resolution as u32,
    );

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = screen[&(x as u16, y as u16)].into();
    }
    imgbuf.save("output.png").unwrap();
}

pub fn render_scene_sdl2(scene: &mut Scene) -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width: u32 = scene.camera.x_resolution.into();
    let height: u32 = scene.camera.y_resolution.into();

    let window = video_subsystem
        .window("rust raytracer", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::RGBA8888, width, height)
        .map_err(|e| e.to_string())?;

    render_frame_scene_sdl2(scene, &mut canvas, &mut texture, width, height)?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            let mut render = false;
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    let last_object = &mut scene.objects.last_mut().unwrap().geometry;
                    last_object.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -5f64,
                    });
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    let last_object = &mut scene.objects.last_mut().unwrap().geometry;
                    last_object.translate(&Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: 5f64,
                    });
                    render = true;
                }
                _ => {}
            }
            if render {
                render_frame_scene_sdl2(scene, &mut canvas, &mut texture, width, height)?;
            }
        }
    }

    Ok(())
}

pub fn render_frame_scene_sdl2(
    scene: &Scene,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &mut sdl2::render::Texture,
    width: u32,
    height: u32,
) -> Result<(), String> {
    let viewport = scene.camera.generate_viewport();
    let screen: HashMap<_, _> = viewport
        .par_iter()
        .map(|view_ray| ((view_ray.x, view_ray.y), cast_ray(&scene, &view_ray.ray)))
        .collect();
    canvas
        .with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.clear();
            for y in 0..scene.camera.y_resolution {
                for x in 0..scene.camera.x_resolution {
                    let sdl2_color: sdl2::pixels::Color = screen[&(x, y)].into();
                    texture_canvas.set_draw_color(sdl2_color);
                    texture_canvas
                        .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                        .unwrap();
                }
            }
        })
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.copy(
        &texture,
        None,
        Some(sdl2::rect::Rect::new(0, 0, width, height)),
    )?;
    canvas.present();

    Ok(())
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
            camera: OrthographicCamera {
                x_resolution: 50u16,
                y_resolution: 25u16,
            },
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
        }));
        let scene = Scene {
            objects: objects,
            lights: lights,
            camera: OrthographicCamera {
                x_resolution: 50u16,
                y_resolution: 25u16,
            },
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
            camera: OrthographicCamera {
                x_resolution: 50u16,
                y_resolution: 25u16,
            },
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
            camera: OrthographicCamera {
                x_resolution: 50u16,
                y_resolution: 25u16,
            },
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
