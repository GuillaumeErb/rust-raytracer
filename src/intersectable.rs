use crate::geometry::Object;
use crate::geometry::Plane;
use crate::geometry::Ray;
use crate::geometry::Sphere;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(ref obj) => obj.intersect(ray),
            Object::Plane(ref obj) => obj.intersect(ray),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l = &self.center - &ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
}

impl Intersectable for Plane {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;
    use crate::Vector3;
    #[test]
    fn sphere_intersection() {
        let sphere = Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };
        let ray = Ray {
            origin: Point3 {
                x: -10f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 1f64,
                y: 0f64,
                z: 0f64,
            },
        };
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection.is_some(), true);
        assert_eq!(intersection.unwrap(), 6f64);
    }

    #[test]
    fn sphere_no_intersection() {
        let sphere = Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };
        let ray = Ray {
            origin: Point3 {
                x: -10f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: -1f64,
                y: 0f64,
                z: 0f64,
            },
        };
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection.is_none(), true);
    }

    #[test]
    fn sphere_casted_inside() {
        let sphere = Sphere {
            center: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };
        let ray = Ray {
            origin: Point3 {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: -1f64,
                y: 0f64,
                z: 0f64,
            },
        };
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection.is_some(), true);
        assert_eq!(intersection.unwrap(), 4f64);
    }
}
