use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Predictions {
    #[serde(rename = "displayNames")]
    display_names: Vec<String>,

    confidences: Vec<f64>,
    bboxes: Vec<Vec<f64>>,
    ids: Vec<String>,
}

#[derive(Debug)]
struct BBox {
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
}

fn main() {
    let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep-predictions.json";
    let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep.jpg";

    let img = image::open(ifname).unwrap();
    let dim = img.dimensions();
    println!("dimensions {:?}", dim);

    let json = fs::read_to_string(pfname).unwrap();
    let ps: Predictions = serde_json::from_str(&json).unwrap();
    let bbox = BBox {
        x_min: (ps.bboxes[0][0] * dim.0 as f64).round() as u32,
        x_max: (ps.bboxes[0][1] * dim.0 as f64).round() as u32,
        y_min: (ps.bboxes[0][2] * dim.1 as f64).round() as u32,
        y_max: (ps.bboxes[0][3] * dim.1 as f64).round() as u32,
    };
    println!("bbox {:?}", bbox);
}
