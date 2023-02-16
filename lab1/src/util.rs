use std::fs;

use crate::algorithm;

pub fn newton(filename: &str, x: f64, n: usize) -> Box<dyn Fn(f64) -> f64> {
    let (xs, ys) = read_xy_from_file(filename);

    algorithm::get_newton_interpolation_func(&xs, &ys, x, n)
}

pub fn hermite(filename: &str, x: f64, n: usize) -> Box<dyn Fn(f64) -> f64> {
    let (xs, ys, dydxs) = read_xydydx_from_file(filename);

    algorithm::get_hermite_interpolation_func(&xs, &ys, &dydxs, x, n)
}

pub fn read_data_from_file(filename: &str) -> Vec<Vec<f64>> {
    let file_contents: String = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut data: Vec<Vec<f64>> = Vec::new();

    for line in &lines[1..] {
        if line.len() > 0 {
            let values: Vec<f64> = line.split(",")
                .into_iter()
                .map(|s| s.parse::<f64>().unwrap())
                .collect();

            data.push(values);
        }
    }

    data
}

pub fn read_xy_from_file(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    let data = read_data_from_file(filename);

    for values in data {
        xs.push(values[0]);
        ys.push(values[1]);
    }

    (xs, ys)
}

pub fn read_xydydx_from_file(filename: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut dydxs: Vec<f64> = Vec::new();

    let data = read_data_from_file(filename);

    for values in data {
        xs.push(values[0]);
        ys.push(values[1]);
        dydxs.push(values[2]);
    }

    (xs, ys, dydxs)
}
