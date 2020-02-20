use crate::engine::SceneObject;
use crate::engine::SceneObjectId;
use crate::geometry::MeshTriangle;
use crate::geometry::Object;
use crate::geometry::Ray;
use crate::geometry::Sphere;
use std::collections::HashSet;

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
}

pub trait AxisAlignedBoundingBoxable {
    fn get_aabb(&self) -> Option<AxisAlignedBoundingBox>;
}

impl AxisAlignedBoundingBoxable for Object {
    fn get_aabb(&self) -> Option<AxisAlignedBoundingBox> {
        match self {
            Object::Sphere(ref obj) => obj.get_aabb(),
            Object::Plane(ref _obj) => None,
            Object::MeshTriangle(ref obj) => obj.get_aabb(),
        }
    }
}

pub fn build_kd_tree(objects: &Vec<SceneObject>) -> KDTree {
    let mut kd_tree = initialize_kd_tree(objects);
    split_kd_tree(&mut kd_tree.tree, &kd_tree.objects_aabb, Direction::X, 0);
    //println!("{:?}", kd_tree.tree);
    kd_tree
}

pub fn initialize_kd_tree(objects: &Vec<SceneObject>) -> KDTree {
    let mut min_x: f64 = std::f64::MAX;
    let mut max_x: f64 = std::f64::MIN;
    let mut min_y: f64 = std::f64::MAX;
    let mut max_y: f64 = std::f64::MIN;
    let mut min_z: f64 = std::f64::MAX;
    let mut max_z: f64 = std::f64::MIN;

    let objects_aabb: Vec<Option<AxisAlignedBoundingBox>> = objects
        .iter()
        .map(|object| object.geometry.get_aabb())
        .collect();

    for object_aabb in objects_aabb.iter() {
        match object_aabb {
            Option::Some(aabb) => {
                if aabb.min_x < min_x {
                    min_x = aabb.min_x;
                }
                if aabb.min_y < min_y {
                    min_y = aabb.min_y;
                }
                if aabb.min_z < min_z {
                    min_z = aabb.min_z;
                }
                if aabb.max_x > max_x {
                    max_x = aabb.max_x;
                }
                if aabb.max_y > max_y {
                    max_y = aabb.max_y;
                }
                if aabb.max_z > max_z {
                    max_z = aabb.max_z;
                }
            }
            None => continue,
        }
    }
    let scene_aabb: AxisAlignedBoundingBox = AxisAlignedBoundingBox {
        min_x: min_x,
        max_x: max_x,
        min_y: min_y,
        max_y: max_y,
        min_z: min_z,
        max_z: max_z,
    };

    let objects_indices: HashSet<SceneObjectId> = objects.iter().map(|object| object.id).collect();

    KDTree {
        objects_aabb: objects_aabb,
        tree: KDTreeNode {
            aabb: scene_aabb,
            objects: objects_indices,
            left: None,
            right: None,
        },
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    X,
    Y,
    Z,
}

impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::X => Direction::Y,
            Direction::Y => Direction::Z,
            Direction::Z => Direction::X,
        }
    }
}

fn split_kd_tree(
    kd_tree: &mut KDTreeNode,
    aabb: &Vec<Option<AxisAlignedBoundingBox>>,
    direction: Direction,
    depth: u8,
) {
    if depth > 5 {
        return;
    }

    let mut left: HashSet<SceneObjectId> = HashSet::new();
    let mut right: HashSet<SceneObjectId> = HashSet::new();
    let left_aabb: AxisAlignedBoundingBox;
    let right_aabb: AxisAlignedBoundingBox;
    let split: f64;
    match direction {
        Direction::X => {
            split = (kd_tree.aabb.min_x + kd_tree.aabb.max_x) / 2f64;

            left_aabb = AxisAlignedBoundingBox {
                min_x: kd_tree.aabb.min_x,
                max_x: split,
                min_y: kd_tree.aabb.min_y,
                max_y: kd_tree.aabb.max_y,
                min_z: kd_tree.aabb.min_z,
                max_z: kd_tree.aabb.max_z,
            };
            right_aabb = AxisAlignedBoundingBox {
                min_x: split,
                max_x: kd_tree.aabb.max_x,
                min_y: kd_tree.aabb.min_y,
                max_y: kd_tree.aabb.max_y,
                min_z: kd_tree.aabb.min_z,
                max_z: kd_tree.aabb.max_z,
            };
        }
        Direction::Y => {
            split = (kd_tree.aabb.min_y + kd_tree.aabb.max_y) / 2f64;

            left_aabb = AxisAlignedBoundingBox {
                min_x: kd_tree.aabb.min_x,
                max_x: kd_tree.aabb.max_x,
                min_y: kd_tree.aabb.min_y,
                max_y: split,
                min_z: kd_tree.aabb.min_z,
                max_z: kd_tree.aabb.max_z,
            };
            right_aabb = AxisAlignedBoundingBox {
                min_x: kd_tree.aabb.min_x,
                max_x: kd_tree.aabb.max_x,
                min_y: split,
                max_y: kd_tree.aabb.max_y,
                min_z: kd_tree.aabb.min_z,
                max_z: kd_tree.aabb.max_z,
            };
        }
        Direction::Z => {
            split = (kd_tree.aabb.min_z + kd_tree.aabb.max_z) / 2f64;

            left_aabb = AxisAlignedBoundingBox {
                min_x: kd_tree.aabb.min_x,
                max_x: kd_tree.aabb.max_x,
                min_y: kd_tree.aabb.min_y,
                max_y: kd_tree.aabb.max_y,
                min_z: kd_tree.aabb.min_z,
                max_z: split,
            };
            right_aabb = AxisAlignedBoundingBox {
                min_x: kd_tree.aabb.min_x,
                max_x: kd_tree.aabb.max_x,
                min_y: kd_tree.aabb.min_y,
                max_y: kd_tree.aabb.max_y,
                min_z: split,
                max_z: kd_tree.aabb.max_z,
            };
        }
    }

    let mut unboud_objects: HashSet<SceneObjectId> = HashSet::new();
    let initial_objects_count = kd_tree.objects.len();

    for &aabb_index in kd_tree.objects.iter() {
        let aabb_option: &Option<AxisAlignedBoundingBox> = &aabb[aabb_index];
        match aabb_option {
            Some(aabb) => match direction {
                Direction::X => {
                    if aabb.min_x > split {
                        right.insert(aabb_index);
                    } else if aabb.max_x < split {
                        left.insert(aabb_index);
                    } else {
                        right.insert(aabb_index);
                        left.insert(aabb_index);
                    }
                }
                Direction::Y => {
                    if aabb.min_y > split {
                        right.insert(aabb_index);
                    } else if aabb.max_y < split {
                        left.insert(aabb_index);
                    } else {
                        right.insert(aabb_index);
                        left.insert(aabb_index);
                    }
                }
                Direction::Z => {
                    if aabb.min_z > split {
                        right.insert(aabb_index);
                    } else if aabb.max_z < split {
                        left.insert(aabb_index);
                    } else {
                        right.insert(aabb_index);
                        left.insert(aabb_index);
                    }
                }
            },
            None => {
                unboud_objects.insert(aabb_index);
            }
        }
    }

    let stable = left.len() == initial_objects_count && right.len() < initial_objects_count;
    if stable {
        return;
    }

    kd_tree.objects = unboud_objects;
    if !left.is_empty() {
        let mut left_kd_tree = KDTreeNode {
            aabb: left_aabb,
            objects: left,
            left: None,
            right: None,
        };
        split_kd_tree(&mut left_kd_tree, aabb, direction.next(), depth + 1);
        kd_tree.left = Some(Box::new(left_kd_tree));
    }

    if !right.is_empty() {
        let mut right_kd_tree = KDTreeNode {
            aabb: right_aabb,
            objects: right,
            left: None,
            right: None,
        };
        split_kd_tree(&mut right_kd_tree, aabb, direction.next(), depth + 1);
        kd_tree.right = Some(Box::new(right_kd_tree));
    }
}

pub struct KDTree {
    pub objects_aabb: Vec<Option<AxisAlignedBoundingBox>>,
    pub tree: KDTreeNode,
}

#[derive(Debug)]
pub struct KDTreeNode {
    pub aabb: AxisAlignedBoundingBox,
    pub objects: HashSet<SceneObjectId>,
    pub left: Option<Box<KDTreeNode>>,
    pub right: Option<Box<KDTreeNode>>,
}

impl KDTree {
    pub fn get_leafs_intersecting(&self, ray: &Ray) -> HashSet<SceneObjectId> {
        let mut leafs = vec![];
        self.tree.fill_leafs_intersecting(ray, &mut leafs);
        let objects: HashSet<SceneObjectId> = leafs
            .iter()
            .map(|node| node.objects.iter())
            .flatten()
            .map(|&x| x)
            .collect();
        objects
    }
}

impl KDTreeNode {
    fn fill_leafs_intersecting<'a>(&'a self, ray: &Ray, leafs: &mut Vec<&'a KDTreeNode>) -> () {
        if self.aabb.intersects(ray) {
            let mut left_leaf = false;
            let mut right_leaf = true;
            match &self.left {
                Some(node) => node.fill_leafs_intersecting(ray, leafs),
                None => left_leaf = true,
            }
            match &self.right {
                Some(node) => node.fill_leafs_intersecting(ray, leafs),
                None => right_leaf = true,
            }
            if left_leaf && right_leaf {
                leafs.push(self);
            }
        }
    }
}

macro_rules! min {
    ($x:expr, $y:expr, $z:expr) => {
        if $y < $x {
            if $z < $y {
                $z
            } else {
                $y
            }
        } else {
            $x
        }
    };
}

macro_rules! max {
    ($x:expr, $y:expr, $z:expr) => {
        if $y > $x {
            if $z > $y {
                $z
            } else {
                $y
            }
        } else {
            $x
        }
    };
}

impl AxisAlignedBoundingBoxable for MeshTriangle {
    fn get_aabb(&self) -> Option<AxisAlignedBoundingBox> {
        let triangle = &self.mesh.triangles[self.triangle_index];
        let a = &self.mesh.vertices[triangle.vertex_a.vertex_index];
        let b = &self.mesh.vertices[triangle.vertex_b.vertex_index];
        let c = &self.mesh.vertices[triangle.vertex_c.vertex_index];
        Some(AxisAlignedBoundingBox {
            min_x: min!(a.x, b.x, c.x),
            max_x: max!(a.x, b.x, c.x),
            min_y: min!(a.y, b.y, c.y),
            max_y: max!(a.y, b.y, c.y),
            min_z: min!(a.z, b.z, c.z),
            max_z: max!(a.z, b.z, c.z),
        })
    }
}

impl AxisAlignedBoundingBoxable for Sphere {
    fn get_aabb(&self) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox {
            min_x: self.center.x - self.radius,
            max_x: self.center.x + self.radius,
            min_y: self.center.y - self.radius,
            max_y: self.center.y + self.radius,
            min_z: self.center.z - self.radius,
            max_z: self.center.z + self.radius,
        })
    }
}

impl AxisAlignedBoundingBox {
    // http://www.ics.uci.edu/~arvo/EECS204/code/latest/aabb.cpp
    pub fn intersects(&self, ray: &Ray) -> bool {
        let mut r: f64;
        let mut s: f64;
        let mut t: f64;
        let mut min = 0.0f64;
        let mut max = std::f64::MAX;

        if ray.direction.x > 0.0 {
            if ray.origin.x > self.max_x {
                return false;
            }
            r = 1.0 / ray.direction.x;
            s = (self.min_x - ray.origin.x) * r;
            if s > min {
                min = s;
            }
            t = (self.max_x - ray.origin.x) * r;
            if t < max {
                max = t;
            }
        } else if ray.direction.x < 0f64 {
            if ray.origin.x < self.min_x {
                return false;
            }
            r = 1.0 / ray.direction.x;
            s = (self.max_x - ray.origin.x) * r;
            if s > min {
                min = s;
            }
            t = (self.min_x - ray.origin.x) * r;
            if t < max {
                max = t;
            }
        } else if ray.origin.x < self.min_x || ray.origin.x > self.max_x {
            return false;
        }

        if min > max {
            return false;
        }

        if ray.direction.y > 0f64 {
            if ray.origin.y > self.max_y {
                return false;
            }
            r = 1.0 / ray.direction.y;
            s = (self.min_y - ray.origin.y) * r;
            if s > min {
                min = s;
            }
            t = (self.max_y - ray.origin.y) * r;
            if t < max {
                max = t;
            }
        } else if ray.direction.y < 0f64 {
            if ray.origin.y < self.min_y {
                return false;
            }
            r = 1.0 / ray.direction.y;
            s = (self.max_y - ray.origin.y) * r;
            if s > min {
                min = s;
            }
            t = (self.min_y - ray.origin.y) * r;
            if t < max {
                max = t;
            }
        } else if ray.origin.y < self.min_y || ray.origin.y > self.max_y {
            return false;
        }

        if min > max {
            return false;
        }

        if ray.direction.z > 0f64 {
            if ray.origin.z > self.max_z {
                return false;
            }
            r = 1.0 / ray.direction.z;
            s = (self.min_z - ray.origin.z) * r;
            if s > min {
                min = s;
            }
            t = (self.max_z - ray.origin.z) * r;
            if t < max {
                max = t;
            }
        } else if ray.direction.z < 0f64 {
            if ray.origin.z < self.min_z {
                return false;
            }
            r = 1.0 / ray.direction.z;
            s = (self.max_z - ray.origin.z) * r;
            if s > min {
                min = s;
            }
            t = (self.min_z - ray.origin.z) * r;
            if t < max {
                max = t;
            }
        } else if ray.origin.z < self.min_z || ray.origin.z > self.max_z {
            return false;
        }

        return min <= max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point3;
    use crate::geometry::Vector3;

    #[test]
    fn aabb_ray_intersection_straight() {
        let aabb = AxisAlignedBoundingBox {
            min_x: -10f64,
            max_x: 10f64,
            min_y: -20f64,
            max_y: 20f64,
            min_z: -5f64,
            max_z: 5f64,
        };
        let mut ray = Ray {
            origin: Point3 {
                x: -50f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 1f64,
                y: 0f64,
                z: 0f64,
            },
        };
        let intersect = aabb.intersects(&ray);
        assert!(intersect);

        ray = Ray {
            origin: Point3 {
                x: -50f64,
                y: 0f64,
                z: 0f64,
            },
            direction: Vector3 {
                x: 1f64,
                y: 1f64,
                z: 0f64,
            }
            .normalize(),
        };
        let intersect = aabb.intersects(&ray);
        assert!(intersect == false);
    }

    #[test]
    fn aabb_ray_intersection_real_usecase1() {
        let aabb = AxisAlignedBoundingBox {
            min_x: -7f64,
            max_x: 5f64,
            min_y: -5f64,
            max_y: 5f64,
            min_z: -5f64,
            max_z: 5f64,
        };
        let ray = Ray {
            origin: Point3 {
                x: 0f64,
                y: 0f64,
                z: -20f64,
            },
            direction: Vector3 {
                x: 0.65728760956494947f64,
                y: 0.3149503129165383f64,
                z: 0.68467459329682245f64,
            }
            .normalize(),
        };
        let intersect = aabb.intersects(&ray);
        assert!(intersect);
    }

    #[test]
    fn aabb_ray_intersection_real_usecase2() {
        let aabb = AxisAlignedBoundingBox {
            min_x: -1f64,
            max_x: 5f64,
            min_y: -5f64,
            max_y: 5f64,
            min_z: -5f64,
            max_z: 5f64,
        };
        let ray = Ray {
            origin: Point3 {
                x: 0f64,
                y: 0f64,
                z: -20f64,
            },
            direction: Vector3 {
                x: -0.15455478293210748,
                y: -0.15455478293210748,
                z: 0.9758204948378663,
            }
            .normalize(),
        };
        let intersect = aabb.intersects(&ray);
        assert!(intersect);
    }
}
