use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::VecDeque;

mod error;
use error::CustomError;

mod point;
use point::Point;

mod edge;
mod algorithm;

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

    let numbers: Vec<Point<f64>> = match parse_file_contents(contents) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    // println!("Input x (f64): ");
    // let x: f64 = match input_f64() {
    //     Ok(value) => value,
    //     Err(e) => return Err(e),
    // };

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
        },
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

        points.push(Point{ x, y });
    }

    return Ok(points);
}

fn input_f64() -> Result<f64, CustomError> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => return Err(CustomError::new(&e.to_string())),
    }

    let x: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(e) => return Err(CustomError::new(&e.to_string())),
    };

    return Ok(x);
}
