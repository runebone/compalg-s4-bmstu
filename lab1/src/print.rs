use crate::table::Table;

pub trait Printable {
    fn print(self);
}

impl Printable for Table {
    fn print(self) {
        for i in 0..self.records.len() {
            let r = &self.records[i];
            println!("{}. x({:4}) y({:8}) dydx({:8})", i + 1, r.x, r.y, r.dydx);
        }
    }
}

impl Printable for Vec<f64> {
    fn print(self) {
        for i in self {
            print!("{} ", i);
        }
        println!();
    }
}
