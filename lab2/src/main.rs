use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read};

mod error;
use error::CustomError;

mod point;
use point::Point;

mod cubic_spline;
use cubic_spline::{LeftNewtonRightZeroEdges, BothNewtonEdges};

mod newton;
mod edge;

fn main() -> Result<(), CustomError> {
    let args: Vec<String> = env::args().collect();
    let mut filename: &str = "./lab2/data.csv";

    if args.len() > 1 {
        filename = &args[1];
    }

    let contents: String = match read_file_contents(filename) {
        Ok(c) => c,
        Err(e) => return Err(CustomError::new(&e.to_string())),
    };

    let data: Vec<Point<f64>> = match parse_file_contents(contents) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    println!("Table:");
    for i in 0..data.len() {
        println!("{:6} {:6}", data[i].x, data[i].y);
    }

    println!("Input x (f64): ");
    let x: f64 = match input_f64() {
        Ok(value) => value,
        Err(e) => return Err(e),
    };
    
    let n = data.len();
    let mut cs = cubic_spline::CubicSpline::new(&data);

    cs.compute();

    {
        let f = cs.func();

        // println!("Spline interp. (Natural) at x = {}: {}", data[0].x, f(data[0].x));
        // println!("Spline interp. (Natural) at x = {}: {}", data[5].x, f(data[5].x));
        // println!("Spline interp. (Natural) at x = {}: {}", data[n - 1].x, f(data[n - 1].x));
        println!("Spline interp. (Natural) at x = {}: {}", x, f(x));
    }

    cs.set_edges_strategy(Box::new(LeftNewtonRightZeroEdges));
    cs.compute();

    {
        let f = cs.func();

        // println!("Spline interp. (LeftNewton) at x = {}: {}", data[0].x, f(data[0].x));
        // println!("Spline interp. (LeftNewton) at x = {}: {}", data[5].x, f(data[5].x));
        // println!("Spline interp. (LeftNewton) at x = {}: {}", data[n - 1].x, f(data[n - 1].x));
        println!("Spline interp. (LeftNewton) at x = {}: {}", x, f(x));
    }

    cs.set_edges_strategy(Box::new(BothNewtonEdges));
    cs.compute();

    {
        let f = cs.func();

        // println!("Spline interp. (BothNewton) at x = {}: {}", data[0].x, f(data[0].x));
        // println!("Spline interp. (BothNewton) at x = {}: {}", data[5].x, f(data[5].x));
        // println!("Spline interp. (BothNewton) at x = {}: {}", data[n - 1].x, f(data[n - 1].x));
        println!("Spline interp. (BothNewton) at x = {}: {}", x, f(x));
    }

    {
        let f = newton::get_newton_interpolation_func(&data, x, 3);

        println!("Newton interp. at x = {}: {}", x, f(x));
    }

    return Ok(());
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // Open the file for reading
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn parse_file_contents(contents: String) -> Result<Vec<Point<f64>>, CustomError> {
    let mut words: VecDeque<&str> = contents
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect();

    // Pop "x,y"-csv line out of words vector
    match words.pop_front() {
        Some(first_word) => {
            if first_word != "x,y" {
                return Err(CustomError::new("Invalid file structure."));
            }
        }
        None => return Err(CustomError::new("Empty file.")),
    }

    let mut numbers: VecDeque<f64> = VecDeque::new();
    for word in &words {
        let number = match word.parse::<f64>() {
            Ok(value) => value,
            Err(e) => return Err(CustomError::new(&e.to_string())),
        };
        numbers.push_back(number);
    }

    if numbers.len() % 2 != 0 {
        return Err(CustomError::new("Not enough data."));
    }

    let mut points: Vec<Point<f64>> = Vec::new();
    while numbers.len() > 0 {
        let x = numbers.pop_front().unwrap();
        let y = numbers.pop_front().unwrap();

        points.push(Point { x, y });
    }

    return Ok(points);
}

fn input_f64() -> Result<f64, CustomError> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => return Err(CustomError::new(&e.to_string())),
    }

    let x: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(e) => return Err(CustomError::new(&e.to_string())),
    };

    return Ok(x);
}
