use image::GenericImageView;
// use itertools::{Group, Itertools};
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

impl BBox {
    fn width(&self) -> u32 {
        self.x_max - self.x_min
    }

    fn height(&self) -> u32 {
        self.y_max - self.y_min
    }
}

fn nearest5(n: u32) -> u32 {
    (((n as f64) / 5.0).round() * 5.0) as u32
}

fn main() {
    // cemetary
    // let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/cemetary-predictions.json";
    // let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/cemetary.jpg";

    // ruined-keep
    // let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep-predictions.json";
    // let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep.jpg";

    // desert
    let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/desert-predictions.json";
    let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/desert.jpg";

    let img = image::open(ifname).unwrap();
    let dim = img.dimensions();
    println!("dimensions {:?}", dim);

    let json = fs::read_to_string(pfname).unwrap();
    let ps: Predictions = serde_json::from_str(&json).unwrap();
    let confidence_threshold = 0.80;
    let bboxes = (0..ps.bboxes.len())
        .take_while(|i| ps.confidences[*i] >= confidence_threshold)
        .map(|i| BBox {
            x_min: (ps.bboxes[i][0] * dim.0 as f64).round() as u32,
            x_max: (ps.bboxes[i][1] * dim.0 as f64).round() as u32,
            y_min: (ps.bboxes[i][2] * dim.1 as f64).round() as u32,
            y_max: (ps.bboxes[i][3] * dim.1 as f64).round() as u32,
        })
        .collect::<Vec<BBox>>();

    let mut i = 0;
    for bbox in bboxes.iter() {
        println!(
            "bbox: width= {}, height= {}, confidence= {}",
            nearest5(bbox.width()),
            nearest5(bbox.height()),
            ps.confidences[i]
        );
        i += 1;
    }

    // TODO: pick the width and height w/ the most instances
    // let widths = bboxes
    //     .iter()
    //     .group_by(|bb| bb.width())
    //     .into_iter()
    //     .map(|g| g.1.collect().len());
    // print!("{}", widths);
}
