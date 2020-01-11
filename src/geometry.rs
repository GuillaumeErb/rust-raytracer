use std::fmt;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
pub const POINT3_ORIGIN: Point3 = Point3 {
    x: 0f64,
    y: 0f64,
    z: 0f64,
};

impl Point3 {
    pub fn add(&self, vector: &Vector3) -> Point3 {
        Point3 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl std::ops::Sub for &Point3 {
    type Output = Vector3;

    fn sub(self, other: &Point3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

pub const POINT2_ORIGIN: Point2 = Point2 { x: 0f64, y: 0f64 };

impl std::ops::Sub for &Point2 {
    type Output = Vector2;

    fn sub(self, other: &Point2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add<&Vector2> for &Point2 {
    type Output = Point2;

    fn add(self, other: &Vector2) -> Point2 {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for &Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Debug)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    MeshTriangle(MeshTriangle),
}

impl Object {
    pub fn get_normal(&self, point: &Point3) -> Vector3 {
        match *self {
            Object::Sphere(ref obj) => obj.get_normal(point),
            Object::Plane(ref obj) => obj.normal,
            Object::MeshTriangle(ref obj) => obj.get_normal(point),
        }
    }

    pub fn translate(&mut self, vector: &Vector3) {
        match *self {
            Object::Sphere(ref mut obj) => obj.center = obj.center.add(vector),
            Object::Plane(ref mut obj) => obj.point = obj.point.add(vector),
            Object::MeshTriangle(ref mut _obj) => (),
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn get_normal(&self, point: &Point3) -> Vector3 {
        (point - &self.center).normalize()
    }
}

impl fmt::Debug for MeshTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MeshTriangle {}", self.triangle_index)
    }
}

pub struct MeshTriangle {
    pub mesh: Arc<Mesh>,
    pub triangle_index: usize,
}

pub struct Mesh {
    pub vertices: Vec<Point3>,
    pub texture_mapping: Vec<Point2>,
    pub normals: Vec<Vector3>,
    pub triangles: Vec<MeshPlainTriangle>,
}

pub fn get_triangles<'a>(mesh: Arc<Mesh>) -> Vec<MeshTriangle> {
    let triangles = &mesh.triangles;
    let mut result: Vec<MeshTriangle> = vec![];
    let mut i = 0;
    for _triangle in triangles {
        result.push(MeshTriangle {
            mesh: mesh.clone(),
            triangle_index: i,
        });
        i += 1;
    }
    result
}

pub fn get_triangle_normal(triangle_mesh: &MeshTriangle, uv: Point2) -> Vector3 {
    let triangle = &triangle_mesh.mesh.triangles[triangle_mesh.triangle_index];
    let n0 = triangle_mesh.mesh.normals[triangle.vertex_a.normal_index];
    let n1 = triangle_mesh.mesh.normals[triangle.vertex_b.normal_index];
    let n2 = triangle_mesh.mesh.normals[triangle.vertex_c.normal_index];

    let u = uv.x;
    let v = uv.y;

    n1.times(u).plus(&n2.times(v)).plus(&n0.times(1f64 - u - v))
}

impl MeshTriangle {
    pub fn get_normal(&self, _point: &Point3) -> Vector3 {
        let triangle = &self.mesh.triangles[self.triangle_index];

        let a = self.mesh.vertices[triangle.vertex_a.vertex_index];
        let b = self.mesh.vertices[triangle.vertex_b.vertex_index];
        let c = self.mesh.vertices[triangle.vertex_c.vertex_index];

        let ab = &b - &a;
        let ac = &c - &a;
        let normal = ab.cross(&ac).normalize();
        normal
    }
}

pub struct MeshPlainTriangle {
    pub vertex_a: MeshVertex,
    pub vertex_b: MeshVertex,
    pub vertex_c: MeshVertex,
}

pub struct MeshVertex {
    pub vertex_index: usize,
    pub texture_index: usize,
    pub normal_index: usize,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mesh")
    }
}

#[derive(Debug)]
pub struct Plane {
    pub point: Point3,
    pub normal: Vector3,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

    pub fn plus(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn minus(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let normalization = 1f64 / self.norm();
        Vector3 {
            x: self.x * normalization,
            y: self.y * normalization,
            z: self.z * normalization,
        }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn symmetry(&self, reference: &Vector3) -> Vector3 {
        reference
            .times(2f64 * self.dot(reference))
            .plus(&(self.times(-1f64)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn times(&self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn plus(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn minus(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let normalization = 1f64 / self.norm();
        Vector2 {
            x: self.x * normalization,
            y: self.y * normalization,
        }
    }
}
