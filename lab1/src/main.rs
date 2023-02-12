use std::env;
use crate::table::Table;
use crate::print::Printable;

mod algorithm;
mod print;
mod table;

/* Задание:
 * Вход: Таблица значений x, y, dydx; степень полинома n; значение аргумента,
 * для которого требуется интерполировать функцию.
 * Выход: Таблица значений y(x) при степенях полинома Ньютона и Эрмита n=1..5
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    
    let t: Table = Table::read_from_file(filename);

    let x: f64 = 0.6;
    let n: usize = 4; // Degree of polynomial
    
    let (s, e): (usize, usize) = algorithm::choose_n_points_from_table(&t, x, n);
    println!("{} {}", s, e);
    println!("{} {}", t.records[s].x, t.records[e].x);

    let vec: Vec<f64> = algorithm::get_vec_of_dd_values(&t, x, n);
    vec.print();
    
    let sum: f64 = algorithm::interpolate_newton(&t, x, n);

    println!("{:.3}", sum);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
