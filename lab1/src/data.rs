use crate::nannou_draw::Model;
// use crate::util;

// pub static mut FILENAME: &str = "./lab1/lec.csv";
// pub static mut X: f64 = 0.6;
// pub static mut N: usize = 5;

// pub fn set_filename(filename: &'static str) {
//     unsafe { FILENAME = filename };
// }

// pub fn set_x(x: f64) {
//     unsafe { X = x };
// }

// pub fn set_n(n: usize) {
//     unsafe { N = n };
// }

pub static mut FUNCS: Vec<(f64, Box<dyn Fn(f64) -> f64>)> = Vec::new();

pub static mut TMP: Vec<Vec<f64>> = Vec::new();

type Tmp = Vec<Vec<f64>>;

pub fn push_tmp(data: Vec<f64>) {
    unsafe { TMP.push(data) };
}

pub fn get_tmp() -> &'static Tmp {
    unsafe { &TMP }
}

pub fn push(data: (f64, Box<dyn Fn(f64) -> f64>)) {
    unsafe { FUNCS.push(data) };
}

pub fn get_model() -> Model {
    Model {
        funcs: unsafe { &mut FUNCS },
    }
}

// pub fn get_model_with_data() -> Model {
//     let filename: &str = unsafe { FILENAME };
//     let x = unsafe { X };
//     let n = unsafe { N };
//     let nfunc = util::newton(filename, x, n);
//     let hfunc = util::hermite(filename, x, n);

//     Model {
//         x,
//         fn_newton: nfunc,
//         fn_hermite: hfunc,
//     }
// }
