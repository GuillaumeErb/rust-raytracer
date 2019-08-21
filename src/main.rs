fn main() {
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
                            red: 0f32,
                            green: 1f32,
                            blue: 0f32,
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
                            red: 1f32,
                            green: 0f32,
                            blue: 0f32,
                        },
                    },
                ),
            })
        );
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
                    y: 2f64.sqrt()/2f64,
                    z: 2f64.sqrt()/2f64,
                },
                material: &Material::ConstantMaterial (
                    ConstantMaterial {
                        color: Color {
                            red: 0f32,
                            green: 0f32,
                            blue: 1f32,
                        }
                    }
                )
            }
        )
    );
    let scene = Scene {
        objects: objects,
        camera: OrthographicCamera {
            x_resolution: 50u16,
            y_resolution: 25u16,
        },
    };

    render_scene(scene);
}

#[derive(Debug)]
pub enum Material {
    ConstantMaterial(ConstantMaterial),
}

impl Material {
    pub fn render_color(&self) -> Color {
        match *self {
            Material::ConstantMaterial(ref m) => m.color
        }
    }
}

#[derive(Debug)]
pub struct ConstantMaterial {
    color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl From<Color> for ansi_term::Color {
    fn from(item: Color) -> Self {
        ansi_term::Color::RGB(
            (item.red * 255f32).round() as u8,
            (item.green * 255f32).round() as u8,
            (item.blue * 255f32).round() as u8,
        )
    }
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
}

const BLACK: Color = Color {
    red: 0f32,
    green: 0f32,
    blue: 0f32,
};

impl Intersectable for Sphere<'_> {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l = &self.center - &ray.origin;
        let t_ca = l.dot(&ray.direction);
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

impl Intersectable for Plane<'_> {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        //println!("{:?}", *self);
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        //println!("normal={:?}", normal);
        //println!("rayDirection={:?}", &ray.direction);
        //println!("Denom={:?}", denom);
        if denom.abs() > 1e-6 {
            let v = &self.point - &ray.origin;
            let distance = v.dot(&normal) / denom;
            //println!("v={:?}", v);
            //println!("Distance={:?}", distance);
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

#[derive(Debug)]
struct Point {
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

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector3,
}

pub struct Scene<'a> {
    objects: Vec<Object<'a>>,
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

pub fn render_scene(scene: Scene) {
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

pub struct Intersection<'a> {
    distance: f64,
    object: &'a Object<'a>,
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    let intersection = scene.objects.iter()
        .filter_map(|object| object.intersect(ray).map(|distance| Intersection { distance:distance, object:object }))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap());

    intersection.map(|i| (*i.object).get_material().render_color()).unwrap_or(BLACK)
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
                            red: 0f32,
                            green: 1f32,
                            blue: 0f32,
                        },
                    },
                ),
            })
        );
        let scene = Scene {
            objects: objects,
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
                red: 0f32,
                green: 1f32,
                blue: 0f32
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
                            red: 0f32,
                            green: 1f32,
                            blue: 0f32,
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
                            red: 1f32,
                            green: 0f32,
                            blue: 0f32,
                        },
                    },
                ),
            })
        );

        let scene = Scene {
            objects: objects,
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
                red: 1f32,
                green: 0f32,
                blue: 0f32
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
                                red: 0f32,
                                green: 0f32,
                                blue: 1f32,
                            }
                        }
                    )
                }
            )
        );
        let scene = Scene {
            objects: objects,
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
                red: 0f32,
                green: 0f32,
                blue: 1f32
            }
        );
    }
}
