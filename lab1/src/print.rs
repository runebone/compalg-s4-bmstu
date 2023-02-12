use crate::table::Table;

pub trait Printable {
    fn print(self);
}

impl Printable for Table {
    fn print(self) {
        let records = self.get_records();
        for i in 0..records.len() {
            let r = &records[i];
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
