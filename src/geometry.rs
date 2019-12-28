#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn add(&self, vector: &Vector3) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
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
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn get_normal(&self, point: &Point) -> Vector3 {
        match *self {
            Object::Sphere(ref obj) => obj.get_normal(point),
            Object::Plane(ref obj) => obj.normal,
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn get_normal(&self, point: &Point) -> Vector3 {
        (point - &self.center).normalize()
    }
}

#[derive(Debug)]
pub struct Plane {
    pub point: Point,
    pub normal: Vector3,
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
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

    pub fn normalize(&self) -> Vector3 {
        let normalization = 1f64 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3 {
            x: self.x * normalization,
            y: self.y * normalization,
            z: self.z * normalization,
        }
    }
}
