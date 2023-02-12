use std::env;
use crate::table::Table;

mod algorithm;
mod print;
mod table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    
    let t: Table = Table::read_from_file(filename);

    let x: f64 = 0.6;
    let n: usize = 4; // Degree of polynomial
    
    let sum: f64 = algorithm::interpolate_newton(&t, x, n);

    println!("{:.3}", sum);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
