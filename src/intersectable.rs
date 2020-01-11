use crate::geometry::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Intersection {
    pub distance: f64,
    pub triangle_u_v: Option<Point2>,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(ref obj) => obj.intersect(ray),
            Object::Plane(ref obj) => obj.intersect(ray),
            Object::MeshTriangle(ref obj) => obj.intersect(ray),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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

        let distance;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        } else if t0 < 0.0 {
            distance = t1;
        } else if t1 < 0.0 {
            distance = t0;
        } else {
            distance = if t0 < t1 { t0 } else { t1 };
        }
        Some(Intersection {
            distance: distance,
            triangle_u_v: None,
        })
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom.abs() > 1e-6 {
            let v = &self.point - &ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(Intersection {
                    distance: distance,
                    triangle_u_v: None,
                });
            }
        }
        None
    }
}

impl Intersectable for MeshTriangle {
    // Moller Trumbore algorithm
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let triangle = &self.mesh.triangles[self.triangle_index];
        let v0 = &self.mesh.vertices[triangle.vertex_a.vertex_index];
        let v1 = &self.mesh.vertices[triangle.vertex_b.vertex_index];
        let v2 = &self.mesh.vertices[triangle.vertex_c.vertex_index];

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;

        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < 1e-6 {
            // remove abs for culling
            return None;
        }

        let inv_det = 1f64 / det;
        let tvec = &ray.origin - &v0;
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0f64 || u > 1f64 {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;

        if v < 0f64 || u + v > 1f64 {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if t < 0f64 {
            return None;
        }
        //println!("Distance {:?}", t);

        Some(Intersection {
            distance: t,
            triangle_u_v: Some(Point2 { x: u, y: v }),
        })
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
        assert_eq!(intersection.unwrap().distance, 6f64);
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
        assert_eq!(intersection.unwrap().distance, 4f64);
    }

    #[test]
    fn triangle_intersection() {
        let mesh = Arc::new(Mesh {
            vertices: vec![
                Point3 {
                    x: 0f64,
                    y: 0f64,
                    z: 0f64,
                },
                Point3 {
                    x: 0f64,
                    y: 2f64,
                    z: 0f64,
                },
                Point3 {
                    x: 2f64,
                    y: 0f64,
                    z: 0f64,
                },
            ],
            texture_mapping: vec![],
            normals: vec![],
            triangles: vec![MeshPlainTriangle {
                vertex_a: MeshVertex {
                    vertex_index: 0,
                    normal_index: 0,
                    texture_index: 0,
                },
                vertex_b: MeshVertex {
                    vertex_index: 1,
                    normal_index: 0,
                    texture_index: 0,
                },
                vertex_c: MeshVertex {
                    vertex_index: 2,
                    normal_index: 0,
                    texture_index: 0,
                },
            }],
        });
        let ray = Ray {
            origin: Point3 {
                x: 0.5f64,
                y: 0.5f64,
                z: -4f64,
            },
            direction: Vector3 {
                x: 0f64,
                y: 0f64,
                z: 1f64,
            },
        };

        let mesh_triangles = get_triangles(mesh);
        let intersection = mesh_triangles[0].intersect(&ray);
        assert_eq!(intersection.is_some(), true);
        assert_eq!(intersection.unwrap().distance, 4f64);
    }
}
