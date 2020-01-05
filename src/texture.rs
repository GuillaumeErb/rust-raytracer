use crate::color::{Color, BLACK, WHITE};
use crate::geometry::{Object, Plane, Point2, Point3, Sphere, Vector3};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Texture {
    pub pixels: Vec<Vec<Color>>,
    pub scale: f64,
    pub offset: Point2,
}

pub fn get_checkboard() -> Vec<Vec<Color>> {
    vec![vec![BLACK, WHITE], vec![WHITE, BLACK]]
}

impl Texture {
    pub fn get_color(&self, object: &Object, point: &Point3) -> Color {
        let coordinate = object.get_2d_coordinate(point);
        let x_float = coordinate.x / self.scale + self.offset.x;
        let y_float = coordinate.y / self.scale + self.offset.y;
        let x = x_float.floor() as usize % self.pixels.len();
        let y = y_float.floor() as usize % self.pixels[0].len();
        self.pixels[x][y]
    }
}

trait Texturable {
    fn get_2d_coordinate(&self, point: &Point3) -> Point2;
}

impl Texturable for Object {
    fn get_2d_coordinate(&self, point: &Point3) -> Point2 {
        match *self {
            Object::Sphere(ref obj) => obj.get_2d_coordinate(point),
            Object::Plane(ref obj) => obj.get_2d_coordinate(point),
        }
    }
}

impl Texturable for Sphere {
    fn get_2d_coordinate(&self, point: &Point3) -> Point2 {
        let hit_vec = point - &self.center;
        Point2 {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x)) / PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() / PI,
        }
    }
}

impl Texturable for Plane {
    fn get_2d_coordinate(&self, point: &Point3) -> Point2 {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.norm() < 1e-6 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = point - &self.point;

        Point2 {
            x: hit_vec.dot(&x_axis),
            y: hit_vec.dot(&y_axis),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;
    use crate::Sphere;

    #[test]
    fn sphere_2d_coordinates() {
        let sphere = Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };

        let mut coordinates = sphere.get_2d_coordinate(&Point3 {
            x: 4f64,
            y: 0f64,
            z: 0f64,
        });
        assert!(
            (coordinates.x - 0.5f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 0.5f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );

        coordinates = sphere.get_2d_coordinate(&Point3 {
            x: -4f64,
            y: 0f64,
            z: 0f64,
        });
        assert!(
            (coordinates.x - 1f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 0.5f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );

        coordinates = sphere.get_2d_coordinate(&Point3 {
            x: 0f64,
            y: 4f64,
            z: 0f64,
        });
        assert!(
            (coordinates.x - 0.5f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 0f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );

        coordinates = sphere.get_2d_coordinate(&Point3 {
            x: 0f64,
            y: -4f64,
            z: 0f64,
        });
        assert!(
            (coordinates.x - 0.5f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 1f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );

        coordinates = sphere.get_2d_coordinate(&Point3 {
            x: 0f64,
            y: 0f64,
            z: 4f64,
        });
        assert!(
            (coordinates.x - 0.75f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 0.5f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );

        coordinates = sphere.get_2d_coordinate(&Point3 {
            x: 0f64,
            y: 0f64,
            z: -4f64,
        });
        assert!(
            (coordinates.x - 0.25f64).abs() < 1e-6,
            "Got x:{}",
            coordinates.x
        );
        assert!(
            (coordinates.y - 0.5f64).abs() < 1e-6,
            "Got y:{}",
            coordinates.y
        );
    }
}
