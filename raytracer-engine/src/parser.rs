use crate::camera::Camera;
use crate::engine::Scene;
use crate::engine::SceneObject;
use crate::geometry::{
    get_triangles, Mesh, MeshPlainTriangle, MeshVertex, Object, Plane, Point2, Point3, Sphere,
    Vector3,
};
use crate::light::{AmbientLight, Light};
use crate::material::Material;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

pub fn parse_obj(filename: String) -> Mesh {
    let mut vertices: Vec<Point3> = vec![];
    let mut texture_mapping: Vec<Point2> = vec![];
    let mut normals: Vec<Vector3> = vec![];
    let mut triangles: Vec<MeshPlainTriangle> = vec![];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let line = line.unwrap();
        let splitted: Vec<_> = line.split_whitespace().collect();
        if splitted.len() == 0 {
            continue;
        }
        match splitted[0] {
            "v" => vertices.push(parse_vertex(splitted)),
            "vt" => texture_mapping.push(parse_vertex_texture(splitted)),
            "vn" => normals.push(parse_normal(splitted)),
            "f" => triangles.push(parse_triangle(splitted)),
            _ => continue,
        }
    }
    Mesh {
        vertices: vertices,
        texture_mapping: texture_mapping,
        normals: normals,
        triangles: triangles,
    }
}

pub fn parse_obj_string(serialized_scene: &str) -> Mesh {
    let mut vertices: Vec<Point3> = vec![];
    let mut texture_mapping: Vec<Point2> = vec![];
    let mut normals: Vec<Vector3> = vec![];
    let mut triangles: Vec<MeshPlainTriangle> = vec![];

    for (index, line) in serialized_scene.split('\n').enumerate() {
        if index == 0 {
            continue;
        }
        let splitted: Vec<_> = line.split_whitespace().collect();
        if splitted.len() == 0 {
            continue;
        }
        match splitted[0] {
            "v" => vertices.push(parse_vertex(splitted)),
            "vt" => texture_mapping.push(parse_vertex_texture(splitted)),
            "vn" => normals.push(parse_normal(splitted)),
            "f" => triangles.push(parse_triangle(splitted)),
            _ => continue,
        }
    }
    Mesh {
        vertices: vertices,
        texture_mapping: texture_mapping,
        normals: normals,
        triangles: triangles,
    }
}

pub fn parse_vertex(line: Vec<&str>) -> Point3 {
    let x = line[1].parse::<f64>().unwrap();
    let y = line[2].parse::<f64>().unwrap();
    let z = line[3].parse::<f64>().unwrap();

    Point3 { x: x, y: y, z: z }
}

pub fn parse_vertex_texture(line: Vec<&str>) -> Point2 {
    let x = line[1].parse::<f64>().unwrap();
    let y = line[2].parse::<f64>().unwrap();

    Point2 { x: x, y: y }
}

pub fn parse_normal(line: Vec<&str>) -> Vector3 {
    let x = line[1].parse::<f64>().unwrap();
    let y = line[2].parse::<f64>().unwrap();
    let z = line[3].parse::<f64>().unwrap();

    Vector3 { x: x, y: y, z: z }
}

pub fn parse_triangle(line: Vec<&str>) -> MeshPlainTriangle {
    let a = parse_face_vertex(line[1]);
    let b = parse_face_vertex(line[2]);
    let c = parse_face_vertex(line[3]);

    MeshPlainTriangle {
        vertex_a: a,
        vertex_b: b,
        vertex_c: c,
    }
}

fn parse_face_vertex(raw_vertex: &str) -> MeshVertex {
    let splitted: Vec<_> = raw_vertex.split('/').collect();
    let vertex = splitted[0].parse::<usize>().unwrap();
    let texture = if splitted.len() > 1 {
        splitted[1].parse::<usize>().unwrap()
    } else {
        1
    };
    let normal = if splitted.len() > 2 {
        splitted[2].parse::<usize>().unwrap()
    } else {
        1
    };

    MeshVertex {
        vertex_index: vertex - 1,
        texture_index: texture - 1,
        normal_index: normal - 1,
    }
}

pub fn deserialize_scene(serialized_scene: &str) -> Scene {
    let serde_scene: SerdeScene = serde_json::from_str(serialized_scene).unwrap();
    let mut scene = Scene {
        objects: deserialize_object(&serde_scene.objects),
        ambient_light: serde_scene.ambient_light,
        lights: serde_scene.lights,
        camera: serde_scene.camera,
    };
    scene
}

pub fn deserialize_object(serde_scene_objects: &Vec<SerdeSceneObject>) -> Vec<SceneObject> {
    let mut result: Vec<SceneObject> = vec![];
    let mut index = 0usize;
    for serde_scene_object in serde_scene_objects.iter() {
        match &serde_scene_object.geometry {
            SerdeObject::Sphere(sphere) => {
                result.push(SceneObject {
                    id: index,
                    geometry: Object::Sphere(sphere.clone()),
                    material: serde_scene_object.material.clone(),
                });
                index += 1;
            }
            SerdeObject::Plane(plane) => {
                result.push(SceneObject {
                    id: index,
                    geometry: Object::Plane(plane.clone()),
                    material: serde_scene_object.material.clone(),
                });
                index += 1;
            }
            SerdeObject::Mesh(mesh) => {
                let mesh = Arc::new(parse_obj_string(&mesh.obj));
                for triangle in get_triangles(mesh) {
                    result.push(SceneObject {
                        id: index,
                        geometry: Object::MeshTriangle(triangle),
                        material: serde_scene_object.material.clone(), // shouldn't be cloned
                    });
                    index += 1;
                }
            }
        };
    }
    result
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerdeScene {
    pub objects: Vec<SerdeSceneObject>,
    pub ambient_light: AmbientLight,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

#[derive(Deserialize, Serialize)]
pub struct SerdeSceneObject {
    pub geometry: SerdeObject,
    pub material: Material,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SerdeObject {
    Sphere(Sphere),
    Plane(Plane),
    Mesh(SerdeMesh),
}

#[derive(Deserialize, Serialize)]
pub struct SerdeMesh {
    pub obj: String,
}
