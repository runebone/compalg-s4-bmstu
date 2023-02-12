use std::fs;

pub struct Record {
    pub x: f64,
    pub y: f64,
    pub dydx: f64,
}

pub struct Table {
    records: Vec<Record>,
}

impl Table {
    pub fn read_from_file(filename: &str) -> Table {
        let mut v: Vec<Record> = Vec::new();

        let file_contents: String = fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = file_contents.split("\n").collect();

        for line in &lines[1..] {
            if line.len() > 0 {
                let values: Vec<f64> = line.split(",")
                                           .into_iter()
                                           .map(|s| s.parse::<f64>().unwrap())
                                           .collect();
                v.push(Record {
                    x: values[0],
                    y: values[1],
                    dydx: values[2],
                });
            }
        }

        Table { records: v }
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }
}
