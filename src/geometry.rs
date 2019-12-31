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
            y: self.y + vector.y,
            z: self.z + vector.z,
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

    pub fn translate(&mut self, vector: &Vector3) {
        match *self {
            Object::Sphere(ref mut obj) => obj.center = obj.center.add(vector),
            Object::Plane(ref mut obj) => obj.point = obj.point.add(vector),
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

    pub fn normalize(&self) -> Vector3 {
        let normalization = 1f64 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3 {
            x: self.x * normalization,
            y: self.y * normalization,
            z: self.z * normalization,
        }
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn symmetry(&self, reference: &Vector3) -> Vector3 {
        reference.times(2f64*self.dot(reference)).plus(&(self.times(-1f64)))
    }
}
