use plotters::prelude::*;
use std::error::Error;

use csv::WriterBuilder;
use csv::{ ReaderBuilder, StringRecord };
use serde::Serialize;
use serde::Deserialize;
use std::str::FromStr;
use std::cmp::min;

const CSV_FILE_PATH: &str = "train.csv";
const H_SIZE: usize = 360;
const V_SIZE: usize = 100;

// #[derive(Copy)]
type ColorMap = Vec<Vec<ColorTrain>>;
type TrainedMap = Vec<(usize, usize)>;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

impl FromStr for ColorTrain {

    type Err = ();

    fn from_str(input: &str) -> Result<ColorTrain, Self::Err> {
        match input {
            "Red" => Ok(ColorTrain::Red),
            "Blue" => Ok(ColorTrain::Blue),
            "Yellow" => Ok(ColorTrain::Yellow),
            "White" => Ok(ColorTrain::White),
            "Black" => Ok(ColorTrain::Black),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut color_map: ColorMap = csv_read().unwrap();
    let trained_colors = map_trained_colors(&color_map);
    
    plot(&color_map, "foo.png");

    for h in 0..H_SIZE {
        for v in 0..V_SIZE {
            let color = &color_map[h][v];
            if color != &ColorTrain::None {continue;}

            color_map[h][v] = calc_result((h, v), &trained_colors);
        }
    }

    plot(&color_map, "knn_result.png");
}

fn calc_result(cord: (usize, usize), maps: &Vec<TrainedMap>) -> ColorTrain {
    let mut distances = vec![];

    for map in maps {
        //verificar a cordenada inicial
        let mut closest_distance = calc_distance(cord, &map[0]);

        for target in map {
            let distance = calc_distance(cord, &target);
            if distance < closest_distance {
                closest_distance = distance;
            }
        }

        distances.push(closest_distance);
    }

    let min_val = distances.iter().min().unwrap();
    let index = distances.iter().position(|&x| x == *min_val).unwrap();
    


    match index {
        0 => ColorTrain::Red,
        1 => ColorTrain::Blue,
        2 => ColorTrain::Yellow,
        _ => ColorTrain::None,
    }
}

fn calc_distance(cord: (usize, usize), target: &(usize, usize)) -> i32 {
    let cord_y = cord.0 as i32;
    let cord_x = cord.1 as i32;

    let target_y = target.0 as i32;
    let target_x = target.1 as i32;

    let diff_x = (cord_x - target_x).abs();

    let y_distance = (cord_y - target_y).abs();

    let y2_distance = (cord_y - 360).abs() + target_y;
    let y3_distance = (target_y - 360).abs() + cord_y;

    //gambiarra
    let around_distance = min(y2_distance, y3_distance);
    let diff_y = min(y_distance, around_distance);

    let ab = (i32::pow(diff_x, 2) + i32::pow(diff_y, 2)) as f64;

    let root = ab.sqrt();
    root as i32
}

fn map_trained_colors(color_map: &ColorMap) -> Vec<TrainedMap> {
    let color_map = color_map;

    let mut redMap: Vec<(usize, usize)> = vec![];
    let mut blueMap: Vec<(usize, usize)> = vec![];
    let mut yellowMap: Vec<(usize, usize)> = vec![];

    for h in 0..H_SIZE {
        for v in 0..V_SIZE {
            let color = &color_map[h][v];
            match color { 
                ColorTrain::Red => {
                   redMap.push((h, v)) 
                }
                ColorTrain::Blue => {
                    blueMap.push((h, v))
                }
                ColorTrain::Yellow => {
                    yellowMap.push((h, v))
                }
                ColorTrain::White => {}
                ColorTrain::Black => {}
                ColorTrain::None => {}
            }
        }
    }

    [redMap, blueMap, yellowMap].to_vec()
}

fn plot(color_map: &ColorMap, path: &str) -> Result<(), Box<dyn Error>> {
    
    let root_drawing_area = BitMapBackend::new(path, (200, 720)).into_drawing_area();
    // And we can split the drawing area into 3x3 grid
    let child_drawing_areas = root_drawing_area.split_evenly((360, 100));
    // Then we fill the drawing area with different color

    for (area, i) in child_drawing_areas.into_iter().zip(0..) {
        let i = i as f64;

        let h_i = i / 100.0;
        let v_i = i % 100.0;

        let h: f64 =  h_i / 360.0;
        let v: f64 = ((100.0 - v_i) / 2.0) / 100.0;

        let hsl = HSLColor(h, 1.0, v);
        let rgb = hsl.rgb();
        let rgb = RGBColor(rgb.0, rgb.1, rgb.2);


        // let opacity = if color_map[h_i as usize][v_i as usize] != ColorTrain::None {1.0} else {0.3};
        // let rgba = rgb.mix(opacity);
        let rgba = rgb.mix(0.5);

        area.fill(&rgba)?;

        match color_map[h_i as usize][v_i as usize] {
            ColorTrain::Red => {
                area.draw_pixel((h as i32, v as i32), &RED)?;
            }
            ColorTrain::Blue => {
                area.draw_pixel((h as i32, v as i32), &BLUE)?;
            }
            ColorTrain::Yellow => {
                area.draw_pixel((h as i32, v as i32), &YELLOW)?;
            }
            ColorTrain::White => {
                area.draw_pixel((h as i32, v as i32), &WHITE)?;
            }
            ColorTrain::Black => {
                area.draw_pixel((h as i32, v as i32), &BLACK)?;
            }
            ColorTrain::None => {
                // area.draw_pixel((h as i32, v as i32), &BLACK)?;
            }
        }
    }
    Ok(())
}

fn csv_read() -> Result<ColorMap, Box<dyn Error>> { 
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(CSV_FILE_PATH)?;

    let mut color_map = vec![vec![ColorTrain::None; V_SIZE]; H_SIZE];

    for (h, result) in rdr.records().enumerate() {
        let record = result?;
        
        let row: Vec<ColorTrain> = record.deserialize(None).unwrap();

        color_map[h] = row;
    }

    Ok(color_map)
}