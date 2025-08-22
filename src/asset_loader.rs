use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

use dashmap::DashMap;
use image::{ImageFormat, Pixel};

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

    let file = build_asset_path(file).expect("Could not find asset");

    let file_path = Path::new(file.as_str());
    if file_path.extension().unwrap() != "obj" {
        panic!("Expected an obj file.");
    }

    let obj_file = File::open(file_path).expect("Cannot open OBJ file.");

    let (vertex_list, face_list) = parse_obj(obj_file).expect("Cannot read a line");
    let vertex_list: Vec<Point3> = vertex_list
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
        if !line.is_empty() {
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

fn build_triangle(f: Face, points: &[Point3]) -> Triangle {
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

/// Checks the env variable ASSET_DIR to find where assets are stored. Otherwise searches
/// for 6 directories up for a folder called assets and the file itself.
fn build_asset_path(asset_filename: &str) -> Option<String> {
    let folder = std::env::var("ASSET_DIR");

    match folder {
        Ok(path) => {
            // Found a path append the filename
            return Some(path + asset_filename);
        }
        Err(_) => {
            // Found no env variable lets search a bit
            if std::fs::exists("assets/".to_owned() + asset_filename).is_ok() {
                return Some("assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../../../assets/".to_owned() + asset_filename);
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct RTWImage {
    colors: DashMap<(usize, usize), Color>,
    image_width: usize,
    image_height: usize,
}

impl RTWImage {
    /// Loads image data from a file in the folder assets
    pub fn new(image_filename: &str) -> RTWImage {
        // Get path to env folder or check a few directories above TODO: should probably do
        // this for all asset loaders so the assets folder can be found
        let image_filename = build_asset_path(image_filename).expect("Could not find the asset");

        // Now build the type based on the extension, and load in the image:
        let format = ImageFormat::from_path(&image_filename).expect("Unsupported filetype");
        let reader = BufReader::new(File::open(image_filename).unwrap());

        let mut image = image::load(reader, format).expect("Cannot read image");
        let image = image.as_mut_rgb8().unwrap();

        // Loop over the image and populate the dashmap
        let image_height = image.height();
        let image_width = image.width();

        let colors = DashMap::with_capacity((image_height * image_width) as usize);

        dbg!(image_width);
        dbg!(image_height);

        for h in 0..image_height {
            for w in 0..image_width {
                let pixel = image.get_pixel(w, h);

                let rgb = pixel.to_rgb();

                if rgb.0 == [46, 73, 2] {
                    eprintln!("Landho");
                    eprint!("r {}", rgb.0[0] as f64 / 255.0);
                    eprint!("g {}", rgb.0[1] as f64 / 255.0);
                    eprintln!("b {}", rgb.0[2] as f64 / 255.0);
                    eprintln!("{w} {h}");
                }
                let r = rgb.0[0] as f64 / 255.0;
                let g = rgb.0[1] as f64 / 255.0;
                let b = rgb.0[2] as f64 / 255.0;

                colors.insert((w as usize, h as usize), Color::new(r, g, b));
            }
        }

        drop(image.to_owned());

        RTWImage {
            colors,
            image_width: image_width as usize,
            image_height: image_height as usize,
        }
    }

    /// Gets the RTW images width
    pub fn width(&self) -> usize {
        self.image_width
    }

    /// Gets the RTW images height
    pub fn height(&self) -> usize {
        self.image_height
    }

    /// Returns the color at an x, y coordinate for the asset. If you are using this
    /// to place a texture you must convert the uv coordinates to x, y coordinates.
    pub fn pixel_data(&self, x: usize, y: usize) -> Color {
        // Should this be how the library works? It seems weird to fix the pixel coords
        // Maybe it should return none if its out of bounds?
        let x = x.clamp(0, self.image_width - 1);
        let y = y.clamp(0, self.image_height - 1);

        self.colors.get(&(x, y)).unwrap().value().clone()
    }
}
