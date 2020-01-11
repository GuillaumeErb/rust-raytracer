use crate::geometry::{Mesh, MeshPlainTriangle, MeshVertex, Point2, Point3, Vector3};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let splitted: Vec<_> = line.split(' ').collect();
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
    let texture = splitted[1].parse::<usize>().unwrap();
    let normal = splitted[2].parse::<usize>().unwrap();

    MeshVertex {
        vertex_index: vertex - 1,
        texture_index: texture - 1,
        normal_index: normal - 1,
    }
}
