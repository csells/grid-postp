use image::GenericImageView;
use itertools::sorted;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
// use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionInstance {
    #[serde(rename = "instance")]
    pub instance: Instance,

    #[serde(rename = "prediction")]
    pub prediction: Prediction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub content: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub ids: Vec<String>,

    #[serde(rename = "displayNames")]
    pub display_names: Vec<String>,

    pub confidences: Vec<f64>,

    pub bboxes: Vec<Vec<f64>>,
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

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_prediction(pi: PredictionInstance) {
    let maptest_path = Path::new("/Users/csells/Downloads/MapsTestFiles");

    let image_filename = Path::new(&pi.instance.content)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let image_path: PathBuf = [maptest_path, Path::new(image_filename)].iter().collect();
    if !image_path.exists() {
        return;
    }

    let img = image::open(image_path).unwrap();
    let dim = img.dimensions();
    // println!("dimensions {:?}", dim);

    let ps = pi.prediction;
    let confidence_max = ps.confidences[0];
    let confidence_min = confidence_max - 0.1;
    let bboxes = (0..ps.bboxes.len())
        .take_while(|i| ps.confidences[*i] >= confidence_min)
        .map(|i| BBox {
            x_min: (ps.bboxes[i][0] * dim.0 as f64).round() as u32,
            x_max: (ps.bboxes[i][1] * dim.0 as f64).round() as u32,
            y_min: (ps.bboxes[i][2] * dim.1 as f64).round() as u32,
            y_max: (ps.bboxes[i][3] * dim.1 as f64).round() as u32,
        })
        .collect::<Vec<BBox>>();

    println!(
        "\n{}: ({}, {}-{})",
        image_filename,
        bboxes.len(),
        confidence_max,
        confidence_min
    );

    let widths = sorted(bboxes.iter().map(|bb| nearest5(bb.width()))).collect::<Vec<_>>();
    let heights = sorted(bboxes.iter().map(|bb| nearest5(bb.height()))).collect::<Vec<_>>();

    let width_groups = widths
        .iter()
        .group_by(|n| **n)
        .into_iter()
        .map(|(ge0, group)| (ge0, group.collect_vec().len()))
        .sorted_by_key(|g| Reverse(g.1))
        .collect::<Vec<_>>();

    let height_groups = heights
        .iter()
        .group_by(|n| **n)
        .into_iter()
        .map(|(ge0, group)| (ge0, group.collect_vec().len()))
        .sorted_by_key(|g| Reverse(g.1))
        .collect::<Vec<_>>();

    let width = width_groups[0].0;
    let height = height_groups[0].0;
    println!("widths= {:?}", width_groups);
    println!("heights= {:?}", height_groups);
    println!("width, height= {}, {}", width, height);
}

fn main() {
    // cemetary
    // let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/cemetary-predictions.json";
    // let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/cemetary.jpg";

    // ruined-keep
    // let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep-predictions.json";
    // let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/ruined-keep.jpg";

    // desert
    // let pfname = "/Users/csells/Downloads/fantasy-maps-geometry/desert-predictions.json";
    // let ifname = "/Users/csells/Downloads/fantasy-maps-geometry/desert.jpg";

    // predictions
    let fname = "/Users/csells/Downloads/MapsTestFiles/BatchPredictionOutput.jsonl";

    let jlines = read_lines(fname).unwrap();
    for wline in jlines {
        let line = wline.unwrap();
        let pi: PredictionInstance = serde_json::from_str(&line).unwrap();
        check_prediction(pi);
    }
}
