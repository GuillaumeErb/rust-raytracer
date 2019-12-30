use crate::Object;
use crate::Plane;
use crate::Ray;
use crate::Sphere;

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
        //println!("Intersection computation \n{:?}\n{:?}", self, ray);
        let l = &self.center - &ray.origin;
        let t_ca = l.dot(&ray.direction);
        if t_ca < 0f64 {
            //println!("None\n");
            None
        } else {
            let d2 = l.dot(&l) - (t_ca * t_ca);
            let r2 = self.radius * self.radius;
            if r2 < d2 {
                //println!("None\n");
                None
            } else {
                let t_hc = (r2 - d2).sqrt();
                let t1 = t_ca - t_hc;
                let t2 = t_ca + t_hc;
                //println!("Some {:?}\n", t1.min(t2));
                Some(t1.min(t2))
            }
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
    use crate::Point;
    use crate::Vector3;
    #[test]
    fn sphere_intersection() {
        let sphere = Sphere {
            center: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };
        let ray = Ray {
            origin: Point {
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
            center: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 4f64,
        };
        let ray = Ray {
            origin: Point {
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
}
