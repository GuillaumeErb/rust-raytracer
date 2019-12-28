use crate::Ray;
use crate::Object;
use crate::Sphere;
use crate::Plane;

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
