extern crate pyegsphsp;

use std::path::Path;
use pyegsphsp::Transform;
use pyegsphsp::combine;
use pyegsphsp::transform;
use pyegsphsp::transform_in_place;
use pyegsphsp::parse_header;
use std::fs::File;
use std::io::prelude::*;
use std::f64::consts;

/// #[allow(dead_code)]
fn identical(path1: &Path, path2: &Path) -> bool {
    let mut file1 = File::open(path1).unwrap();
    let mut file2 = File::open(path2).unwrap();
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    file1.read_to_end(&mut buf1).unwrap();
    file2.read_to_end(&mut buf2).unwrap();
    buf1.as_slice() == buf2.as_slice()
}

#[test]
fn first_file_header_correct() {
    let path = Path::new("test_data/first.egsphsp");
    let header = parse_header(path).unwrap();
    assert!(header.record_length == 28);
    assert!(header.total_particles == 352, format!("Total particles incorrect, found {:?}", header.total_particles));
    assert!(header.total_photons == 303, format!("Total photons incorrect, found {:?}", header.total_photons));
    assert!(header.max_energy - 0.1988 < 0.0001, format!("Max energy incorrect, found {:?}", header.max_energy));
    assert!(header.min_energy - 0.0157 < 0.0001, format!("Min energy incorrect, found {:?}", header.min_energy));
    assert!(header.total_particles_in_source - 100.0 < 0.0001, format!("Total particles in source incorrect, found {:?}", header.total_particles_in_source));
    // open the first one and make sure the entries are valid
}

#[test]
fn second_file_header_correct() {
    let path = Path::new("test_data/second.egsphsp");
    let header = parse_header(path).unwrap();
    assert!(header.record_length == 28);
    assert!(header.total_particles == 352, format!("Total particles incorrect, found {:?}, header.total_particles", header.total_particles));
    assert!(header.total_photons == 303, format!("Total photons incorrect, found {:?}", header.total_photons));
    assert!(header.max_energy - 0.1988 < 0.0001, format!("Max energy incorrect, found {:?}", header.max_energy));
    assert!(header.min_energy - 0.0157 < 0.0001, format!("Min energy incorrect, found {:?}", header.min_energy));
    assert!(header.total_particles_in_source - 100.0 < 0.0001, format!("Total particles in source incorrect, found {:?}", header.total_particles_in_source));
    // open the first one and make sure the entries are valid
}

#[test]
fn combined_file_header_correct() {
    let path = Path::new("test_data/combined.egsphsp");
    let header = parse_header(path).unwrap();
    assert!(header.record_length == 28);
    assert!(header.total_particles == 352 * 2, format!("Total particles incorrect, found {:?}, header.total_particles", header.total_particles));
    assert!(header.total_photons == 303 * 2, format!("Total photons incorrect, found {:?}", header.total_photons));
    assert!(header.max_energy - 0.1988 < 0.0001, format!("Max energy incorrect, found {:?}", header.max_energy));
    assert!(header.min_energy - 0.0157 < 0.0001, format!("Min energy incorrect, found {:?}", header.min_energy));
    assert!(header.total_particles_in_source - 100.0 * 2.0 < 0.0001, format!("Total particles in source incorrect, found {:?}", header.total_particles_in_source));
    // open the first one and make sure the entries are valid
}

#[test]
fn combine_operation_matches_beamdp() {
    let input_paths = vec![Path::new("test_data/first.egsphsp"), Path::new("test_data/second.egsphsp")];
    let output_path = Path::new("test_data/output_combined.egsphsp");
    let expected_path = Path::new("test_data/combined.egsphsp");
    combine(&input_paths, output_path, false).unwrap();
    assert!(identical(output_path, expected_path));
}

#[test]
fn translate_operation() {
    let input_path = Path::new("test_data/first.egsphsp");
    let output_path = Path::new("test_data/translated.egsphsp");
    let x = 5.0;
    let y = 5.0;
    let mut matrix = [[0.0; 3]; 3];
    Transform::translation(&mut matrix, x, y);
    transform(input_path, output_path, &matrix).unwrap();
    Transform::translation(&mut matrix, -x, -y);
    transform_in_place(output_path, &matrix).unwrap();
    assert!(identical(input_path, output_path));
}

#[test]
fn rotate_operation() {
    let input_path = Path::new("test_data/first.egsphsp");
    let output_path = Path::new("test_data/rotated.egsphsp");
    let mut matrix = [[0.0; 3]; 3];
    Transform::rotation(&mut matrix, consts::PI as f32);
    transform(input_path, output_path, &matrix).unwrap();
    Transform::rotation(&mut matrix, consts::PI as f32);
    transform_in_place(output_path, &matrix).unwrap();
    assert!(identical(input_path, output_path));
}

#[test]
fn reflect_operation() {
    let input_path = Path::new("test_data/first.egsphsp");
    let output_path = Path::new("test_data/reflected.egsphsp");
    let mut matrix = [[0.0; 3]; 3];
    Transform::reflection(&mut matrix, 1.0, 0.0);
    transform(input_path, output_path, &matrix).unwrap();
    Transform::translation(&mut matrix, 1.0, 0.0);
    transform_in_place(output_path, &matrix).unwrap();
    assert!(identical(input_path, output_path));
}

