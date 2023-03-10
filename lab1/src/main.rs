use std::env;

use crate::nannou_draw::{model, update, view};

mod util;
mod data;
mod algorithm;
mod nannou_draw;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename: &str = "./lab1/data.csv";

    if args.len() > 1 {
        filename = &args[1];
    }

    let x: f64 = 0.675;
    let n: usize = 5;

    println!("Задание 1 (x = {:.3})", x);
    {
        println!("{:>2} | {:>9} | {:>9}", "n", "Newton", "Hermite");
        for n in 1..=5 {
            let fn_newton = util::newton(filename, x, n);
            let fn_hermite = util::hermite(filename, x, n);

            println!("{:2} | {:9.6} | {:9.6}", n, fn_newton(x), fn_hermite(x));
        }

        let fn_newton = util::newton(filename, x, 5);
        let fn_hermite = util::hermite(filename, x, 5);

        data::push((x, fn_newton));
        data::push((x, fn_hermite));
    }
    println!();

    println!("Задание 2 (n = {})", n);
    {
        let (xs, ys, dydxs) = util::read_xydydx_from_file(filename);
        let fn_newton_inv = algorithm::get_newton_interpolation_func(&ys, &xs, 0.0, n);

        let dxdys: Vec<f64> = dydxs.iter().map(|x| 1.0 / x).collect();
        let fn_hermite_inv = algorithm::get_hermite_interpolation_func(&ys, &xs, &dxdys, 0.0, n);

        let x_newton = fn_newton_inv(0.0);
        let x_hermite = fn_hermite_inv(0.0);

        println!("Newton: {:.6}", x_newton);
        println!("Hermite: {:.6}", x_hermite);
        println!();

        println!("{:>2} | {:>9} | {:>9} (x_newton = {:.3})", "n", "Newton", "Hermite", x_newton);
        for n in 1..=5 {
            let fn_newton = util::newton(filename, x_newton, n);
            let fn_hermite = util::hermite(filename, x_newton, n);

            println!("{:2} | {:9.6} | {:9.6}", n, fn_newton(x_newton), fn_hermite(x_newton));
        }

        println!();

        println!("{:>2} | {:>9} | {:>9} (x_hermite = {:.3})", "n", "Newton", "Hermite", x_hermite);
        for n in 1..=5 {
            let fn_newton = util::newton(filename, x_hermite, n);
            let fn_hermite = util::hermite(filename, x_hermite, n);

            println!("{:2} | {:9.6} | {:9.6}", n, fn_newton(x_hermite), fn_hermite(x_hermite));
        }
    }
    println!();

    println!("Задание 3");
    {
        let x_interp = 0.78;
        let n = 5;

        let (xs1, ys1) = util::read_xy_from_file("./lab1/3_1.csv");
        let (xs2, ys2) = util::read_xy_from_file("./lab1/3_2.csv");

        // Interpolated ys from the 2nd table
        let mut ys2i: Vec<f64> = Vec::new();

        // Interpolate ys2 based on xs from the 1st table
        let interp_func = algorithm::get_newton_interpolation_func(&xs2, &ys2, x_interp, n);
        for x in &xs1 {
            ys2i.push(interp_func(*x));
        }

        // Get ys_diff values to inverse-interpolate it in 0.0 and find root
        let mut ys_diff: Vec<f64> = Vec::new();
        for i in 0..xs1.len() {
            ys_diff.push(ys2i[i] - ys1[i]);
        }

        let func = algorithm::get_newton_interpolation_func(&ys_diff, &xs1, 0.0, n);
        let root: f64 = func(0.0);

        data::push((0.0, func));

        data::push_tmp(xs1.to_vec());
        data::push_tmp(ys1.to_vec());
        data::push_tmp(xs2.to_vec());
        data::push_tmp(ys2.to_vec());
        data::push_tmp(ys_diff.to_vec());

        let f1 = algorithm::get_newton_interpolation_func(&xs1, &ys1, x_interp, n);
        let f2 = interp_func;
        let f3 = algorithm::get_newton_interpolation_func(&xs1, &ys_diff, x_interp, n);

        let y = f1(root);
        dbg!(y);
        data::push((x_interp, f1));
        data::push((x_interp, f2));
        data::push((x_interp, f3));

        dbg!(xs1, ys_diff);
        dbg!(root);
    }
    println!();

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

