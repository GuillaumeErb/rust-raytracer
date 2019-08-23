fn main() {
    let mut objects: Vec<Object> = vec![];
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -70f64,
                },
                radius: 200f64,
                material: &Material::LambertMaterial (
                    LambertMaterial {
                        color: Color {
                            red: 0f64,
                            green: 1f64,
                            blue: 0f64,
                        },
                    },
                ),
            })
        );
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 40f64,
                    y: 20f64,
                    z: -10f64,
                },
                radius: 150f64,
                material: &Material::LambertMaterial (
                    LambertMaterial {
                        color: Color {
                            red: 1f64,
                            green: 0f64,
                            blue: 0f64,
                        },
                    },
                ),
            })
        );
        objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 350f64,
                    y: 0f64,
                    z: -70f64,
                },
                radius: 50f64,
                material: &Material::LambertMaterial (
                    LambertMaterial {
                        color: Color {
                            red: 0f64,
                            green: 0f64,
                            blue: 1f64,
                        },
                    },
                ),
            })
        );
        /*
    objects.push(
        Object::Plane (
            Plane {
                point: Point {
                    x: 0f64,
                    y: 5f64,
                    z: -50f64,
                },
                normal: Vector3 {
                    x: 0f64,
                    y: -2f64.sqrt()/2f64,
                    z: -2f64.sqrt()/2f64,
                },
                material: &Material::LambertMaterial (
                    LambertMaterial {
                        color: Color {
                            red: 0f64,
                            green: 0f64,
                            blue: 1f64,
                        }
                    }
                )
            }
        )
    );*/
    let mut lights: Vec<Light> = vec![];
    lights.push(
        Light::DirectionalLight(
            DirectionalLight {
                direction: Vector3 {
                    x: -1f64,
                    y: 0f64,
                    z: 0f64,
                }.normalize(),
            },
        ),
    );
    lights.push(
        Light::DirectionalLight(
            DirectionalLight {
                direction: Vector3 {
                    x: 0f64,
                    y: 0f64,
                    z: 1f64,
                }.normalize(),
            },
        ),
    );
    let scene = Scene {
        objects: objects,
        lights: lights,
        camera: OrthographicCamera {
            //x_resolution: 2u16,
            //y_resolution: 2u16,
            x_resolution: 1000u16,
            y_resolution: 800u16,
        },
    };

    render_scene(scene);
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
        let normal = intersection.object.get_normal(&point);
        let mut diffuse_lights = 0f64;
        for light in &scene.lights  {
            if is_in_shadow(&point, &light, scene) {
                //println!("I'm in the dark");
                continue;
            }
            diffuse_lights += normal.dot(&light.get_direction().times(-1f64)).max(0f64);
            //println!("diffuse_lights={:?}", diffuse_lights);
            //println!("normal={:?}", normal);
            //println!("lightDirection={:?}", light.get_direction());
        }
        self.color.times(diffuse_lights/*.powi(5)*5f64*/)
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
            red : (self.red * scalar).min(1f64).max(0f64),
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
        ((item.blue as f32) * 255.0) as u8])
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
    //println!("Light Direction:{:?}", light_direction);
    //println!("Shadow ray:{:?}", shadow_ray);
    
    for object in scene.objects.iter() {
        //println!("Intersection with {:?} at {:?}", object, object.intersect(&shadow_ray));
    }
    scene.objects.iter()
        .filter_map(|object| object.intersect(&shadow_ray)).any(|_d| true)
}

#[derive(Debug)]
pub struct DirectionalLight {
    direction: Vector3,
}

#[derive(Debug)]
struct Sphere<'a> {
    center: Point,
    radius: f64,
    material: &'a Material,
}

#[derive(Debug)]
struct Plane<'a> {
    point: Point,
    normal: Vector3,
    material: &'a Material,
}

#[derive(Debug)]
enum Object <'a>{
    Sphere(Sphere<'a>),
    Plane(Plane<'a>),
}

impl Object<'_> {
    pub fn get_material(&self) -> &Material {
        match *self {
            Object::Sphere(ref obj) => obj.material,
            Object::Plane(ref obj) => obj.material,
        }
    }

    pub fn get_normal(&self, point: &Point) -> Vector3 {
        match *self {
            Object::Sphere(ref obj) => obj.get_normal(point),
            Object::Plane(ref obj) => obj.normal,
        }
    }
}

impl Sphere<'_> {
    pub fn get_normal(&self, point: &Point) -> Vector3 {
        //println!("  Intersection Point:{:?}", point);
        //println!("  center:{:?}", self.center);
        (point - &self.center).normalize()
    }
}

const BLACK: Color = Color {
    red: 0f64,
    green: 0f64,
    blue: 0f64,
};

impl Intersectable for Sphere<'_> {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l = &self.center - &ray.origin;
        let t_ca = l.dot(&ray.direction);
        if t_ca < 0f64 {
            None
        } else {
            let d2 = l.dot(&l) - (t_ca * t_ca);
            let r2 = self.radius * self.radius;
            if r2 < d2 {
                None
            } else {
                let t_hc = (r2 - d2).sqrt();
                let t1 = t_ca - t_hc;
                let t2 = t_ca + t_hc;
                Some(t1.min(t2))
            }
        }
    }
}

impl Intersectable for Plane<'_> {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom.abs() > 1e-6 {
            let v = &self.point - &ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}

impl Intersectable for Object<'_> {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(ref obj) => obj.intersect(ray),
            Object::Plane(ref obj) => obj.intersect(ray),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl std::ops::Sub for &Point {
    type Output = Vector3;

    fn sub(self, other: &Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn add(&self, vector: &Vector3) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn times(&self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn normalize(&self) -> Vector3 {
        //println!("  To normalize {:?}", self);
        let normalization = 1f64/(self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        //println!("  Normalization {:?}", normalization);
        Vector3 {
            x: self.x * normalization,
            y: self.y * normalization,
            z: self.z * normalization,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector3,
}

pub struct Scene<'a> {
    objects: Vec<Object<'a>>,
    lights: Vec<Light>,
    camera: OrthographicCamera,
}

pub struct OrthographicCamera {
    x_resolution: u16,
    y_resolution: u16,
}

pub fn create_view_ray(x: u16, y: u16, camera: &OrthographicCamera) -> Ray {
    let x_shift = camera.x_resolution as f64 / 2f64;
    let y_shift = camera.y_resolution as f64 / 2f64;
    Ray {
        origin: Point {
            x: x as f64 - x_shift,
            y: y as f64 - y_shift,
            z: 0f64,
        },
        direction: Vector3 {
            x: 0f64,
            y: 0f64,
            z: -1f64,
        },
    }
}

pub fn render_scene_console(scene: Scene) {
    for y in 0..scene.camera.y_resolution {
        for x in 0..scene.camera.x_resolution {
            let ray = create_view_ray(x, y, &scene.camera);
            let pixel_color = cast_ray(&scene, &ray);
            let ansi_color: ansi_term::Color = pixel_color.into();
            print!("{}", ansi_color.paint("█"));
        }
        println!();
    }
}

pub fn render_scene(scene: Scene) {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(scene.camera.x_resolution as u32, scene.camera.y_resolution as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let ray = create_view_ray(x as u16, y as u16, &scene.camera);
            let pixel_color = cast_ray(&scene, &ray);
            *pixel = pixel_color.into();
    }
    imgbuf.save("output.png").unwrap();
}


pub struct Intersection<'a> {
    distance: f64,
    object: &'a Object<'a>,
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    let intersection = scene.objects.iter()
        .filter_map(|object| object.intersect(ray).map(|distance| Intersection { distance:distance, object:object }))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());

    intersection.map(|i| (*i.object).get_material().render_color(ray, &i, &scene)).unwrap_or(BLACK)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn single_sphere() {
        let mut objects: Vec<Object> = vec![];
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
                material: &Material::ConstantMaterial (
                    ConstantMaterial {
                        color: Color {
                            red: 0f64,
                            green: 1f64,
                            blue: 0f64,
                        },
                    },
                ),
            })
        );
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
        let mut objects: Vec<Object> = vec![];
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
                material: &Material::LambertMaterial (
                    LambertMaterial {
                        color: Color {
                            red: 0f64,
                            green: 1f64,
                            blue: 0f64,
                        },
                    },
                ),
            })
        );
        let mut lights: Vec<Light> = vec![];
        lights.push(
            Light::DirectionalLight(
                DirectionalLight {
                    direction: Vector3 {
                        x: 0f64,
                        y: 0f64,
                        z: -1f64,
                    }.normalize(),
                },
            ),
        );
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
        let mut objects: Vec<Object> = vec![];
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -50f64,
                },
                radius: 10f64,
                material: &Material::ConstantMaterial (
                    ConstantMaterial {
                        color: Color {
                            red: 0f64,
                            green: 1f64,
                            blue: 0f64,
                        },
                    },
                ), 
            })
        );
    objects.push(
        Object::Sphere (
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: -45f64,
                },
                radius: 6f64,
                material: &Material::ConstantMaterial (
                    ConstantMaterial {
                        color: Color {
                            red: 1f64,
                            green: 0f64,
                            blue: 0f64,
                        },
                    },
                ),
            })
        );

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
        let mut objects: Vec<Object> = vec![];
        objects.push(
            Object::Plane (
                Plane {
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
                    material: &Material::ConstantMaterial (
                        ConstantMaterial {
                            color: Color {
                                red: 0f64,
                                green: 0f64,
                                blue: 1f64,
                            }
                        }
                    )
                }
            )
        );
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
