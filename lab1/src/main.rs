// use std::env;

use crate::nannou_draw::{model, update, view};

mod util;
mod algorithm;
mod nannou_draw;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename: &str = &args[1];
    
    // let n = util::newton(filename, 0.6, 4);
    // let h = util::hermite(filename, 0.6, 4);

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
}

