extern crate nannou;
use nannou::prelude::*;

use crate::util;

pub struct Model {
    x: f64,
    // n: usize,
    fn_newton: Box<dyn Fn(f64) -> f64>,
    fn_hermite: Box<dyn Fn(f64) -> f64>,
}

pub fn model(_app: &App) -> Model {
    let filename: &str = "/home/human/University/compalg/lab1/lec.csv";
    let x = 0.6;
    let n = 5;
    let nfunc = util::newton(filename, x, n);
    let hfunc = util::hermite(filename, x, n);

    Model {
        x,
        // n,
        fn_newton: nfunc,
        fn_hermite: hfunc,
    }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background.
    draw.background().color(BLACK);

    let win = app.window_rect();
    let _t = app.time;

    let nfunc = &model.fn_newton;
    let hfunc = &model.fn_hermite;

    // Decide on a number of points and a weight.
    let n_points = 1000;
    let weight = 1.0;
    let lx = -3.0;
    let rx = 3.0;
    let s = 200.0;

    let mx = model.x.to_f32().unwrap();
    let yn = |x: f32| nfunc(x.to_f64().unwrap()).to_f32().unwrap();
    let yh = |x: f32| hfunc(x.to_f64().unwrap()).to_f32().unwrap();

    let nvertices = (0..n_points)
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, lx, rx);
            pt2(s * x, s * yn(x))
        });

    let hvertices = (0..n_points)
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, lx, rx);
            pt2(s * x, s * yh(x))
        });

    let cvertices = (0..n_points)
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, lx, rx);
            pt2(s * x, s * (PI / 2.0 * x).cos())
        });

    draw.line()
        .start(pt2(win.left(), 0.0))
        .end(pt2(win.right(), 0.0))
        .weight(1.0)
        .color(WHITE);

    draw.line()
        .start(pt2(0.0, win.top()))
        .end(pt2(0.0, win.bottom()))
        .weight(1.0)
        .color(WHITE);

    let mut i = 1.0;
    let grid_color = srgba(0.2, 0.2, 0.2, 1.0);
    while i * s <= win.top() {
        draw.line()
            .start(pt2(win.left(), i * s))
            .end(pt2(win.right(), i * s))
            .weight(1.0)
            .color(grid_color);

        draw.line()
            .start(pt2(win.left(), -(i * s)))
            .end(pt2(win.right(), -(i * s)))
            .weight(1.0)
            .color(grid_color);

        i += 1.0;
    }
    
    i = 1.0;
    while i * s <= win.right() {
        draw.line()
            .start(pt2(i * s, win.top()))
            .end(pt2(i * s, win.bottom()))
            .weight(1.0)
            .color(grid_color);

        draw.text(&i.to_string()).x_y(i * s, -5.0);

        draw.line()
            .start(pt2(-(i * s), win.top()))
            .end(pt2(-(i * s), win.bottom()))
            .weight(1.0)
            .color(grid_color);

        draw.text(&(-i).to_string()).x_y(-(i * s), -5.0);

        i += 1.0;
    }

    draw.line()
        .start(pt2(s * mx, win.top()))
        .end(pt2(s * mx, win.bottom()))
        .weight(1.0)
        .color(GREY);

    draw.text(&(mx).to_string()).x_y(s * mx, -5.0);

    // Draw the polyline as a stroked path.
    draw.polyline()
        .weight(weight)
        .join_round()
        .points(nvertices)
        .color(srgba(1.0, 0.0, 0.0, 1.0));

    draw.polyline()
        .weight(weight)
        .join_round()
        .points(hvertices)
        .color(srgba(0.0, 0.0, 1.0, 1.0));

    draw.polyline()
        .weight(weight)
        .join_round()
        .points(cvertices)
        .color(srgba(0.0, 1.0, 0.0, 1.0));

    draw.ellipse()
        .color(WHITE)
        .w(weight + 2.0)
        .h(weight + 2.0)
        .x_y(s * mx, s * (PI / 2.0 * mx).cos());
    // draw.text(&((PI / 2.0 * mx).cos()).to_string()).x_y(25.0 + s * mx + 5.0, s * (PI / 2.0 * mx).cos());

    draw.ellipse()
        .color(WHITE)
        .w(weight + 2.0)
        .h(weight + 2.0)
        .x_y(s * mx, s * yn(mx));
    // draw.text(&(yn(mx)).to_string()).x_y(25.0 + s * mx, s * yn(mx));

    draw.ellipse()
        .color(WHITE)
        .w(weight + 2.0)
        .h(weight + 2.0)
        .x_y(s * mx, s * yh(mx));
    // draw.text(&(yh(mx)).to_string()).x_y(25.0 + s * mx, s * yh(mx));


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

