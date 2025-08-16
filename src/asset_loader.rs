use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

use crate::{
    material::{Lambertian, Materials},
    objects::{HitList, Hittables, Triangle},
    util::{Color, Point3},
};

/// Loads an object file using the standard Wavefront
/// OBJ format. Returns a Hitlist containing the entire
/// model.
///
/// # Panics
/// This function panics if it encounters any face with
/// more than 3 vertices, or if it is an invalid file extension
/// or if the file is not openable.
pub fn load_obj(file: &str, scale: f64, shift: Point3) -> HitList {
    let mut model = HitList::default();

    let file_path = Path::new(file);
    if file_path.extension().unwrap() != "obj" {
        panic!("Expected an obj file.");
    }

    let obj_file = File::open(file_path).expect("Cannot open OBJ file.");

    let (vertex_list, face_list) = parse_obj(obj_file).expect("Cannot read a line");
    let vertex_list = vertex_list
        .iter()
        .map(|p| scale * p.clone() + shift.clone())
        .collect();

    for face in face_list {
        let triangle = build_triangle(face, &vertex_list);
        //dbg!(&triangle);
        model.add(Hittables::Triangle(triangle));
    }

    model
}

struct Face {
    a_index: usize,
    b_index: usize,
    c_index: usize,
}

impl Face {
    fn new(a_index: usize, b_index: usize, c_index: usize) -> Face {
        Face {
            a_index,
            b_index,
            c_index,
        }
    }
}

/// Todo parse in textures from obj file for now they will all be cyan
fn parse_obj(obj: File) -> Result<(Vec<Point3>, Vec<Face>), Error> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    let reader = BufReader::new(obj);

    for line_result in reader.lines() {
        let line = line_result?;

        let line: Vec<&str> = line.split_whitespace().collect();
        if line.len() > 0 {
            match line[0] {
                "v" => {
                    vertices.push(parse_vertex(line[1..].to_vec()));
                }
                "f" => {
                    faces.push(parse_face(line[1..].to_vec()));
                }
                _ => {
                    panic!("Unsupported OBJ file");
                }
            }
        } else {
            eprintln!("Parsed OBJ file!");
        }
    }

    Ok((vertices, faces))
}

fn parse_vertex(coords: Vec<&str>) -> Point3 {
    assert!(
        coords.len() == 3,
        "Invalid number of coordinates for a vertex"
    );

    let x: f64 = coords[0]
        .parse()
        .expect("Invalid OBJ file. Expected a floating point value for a vertex.");
    let y: f64 = coords[1]
        .parse()
        .expect("Invalid OBJ file. Expected a floating point value for a vertex.");
    let z: f64 = coords[2]
        .parse()
        .expect("Invalid OBJ file. Expected a floating point value for a vertex.");

    Point3::new(x, y, z)
}

fn parse_face(face_points: Vec<&str>) -> Face {
    assert!(
        face_points.len() == 3,
        "The asset loader only supports triangularized images, please triangulate the image then try again"
    );

    let a_index: usize = face_points[0]
        .parse()
        .expect("Invalid OBJ file, expected an index describing the face");

    let b_index: usize = face_points[1]
        .parse()
        .expect("Invalid OBJ file, expected an index describing the face");

    let c_index: usize = face_points[2]
        .parse()
        .expect("Invalid OBJ file, expected an index describing the face");

    Face::new(a_index, b_index, c_index)
}

fn build_triangle(f: Face, points: &Vec<Point3>) -> Triangle {
    // Obj faces are 1 based
    let a = points[f.a_index - 1].clone();
    let b = points[f.b_index - 1].clone();
    let c = points[f.c_index - 1].clone();

    Triangle::new(
        a,
        b,
        c,
        Materials::Lambertian(Lambertian::new_from_color(Color::new(0.8, 0.0, 0.8), 1.0)),
    )
}
