extern crate nannou;
use nannou::prelude::*;

use crate::data;

type MyFunc = Box<dyn Fn(f64) -> f64>;

// TODO: clean-up this hell
// TODO: add egui
pub struct Model {
    // pub x: f64,
    // pub fn_newton: Box<dyn Fn(f64) -> f64>,
    // pub fn_hermite: Box<dyn Fn(f64) -> f64>,
    pub funcs: &'static mut Vec<(f64, Box<dyn Fn(f64) -> f64>)>,
}

impl Model {
    pub fn push(self: &mut Self, data: (f64, Box<dyn Fn(f64) -> f64>)) {
        self.funcs.push(data);
    }

    pub fn get_data(self: &Self) -> &Vec<(f64, Box<dyn Fn(f64) -> f64>)> {
        &self.funcs
    }
}

pub fn model(_app: &App) -> Model {
    // data::get_model_with_data()
    data::get_model()
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background.
    draw.background().color(BLACK);

    let win = app.window_rect();
    let _t = app.time;

    // Decide on a number of points and a weight.
    let n_points = 1000;
    let lx = -3.0;
    let rx = 3.0;
    let s = 200.0;

    draw_grid(&draw, &win, s);

    let d = model.get_data();
    let df = |f, color| draw_func(&draw, f, n_points, s, lx, rx, color);
    // df(&d[2].1, srgba(0.0, 1.0, 0.0, 1.0));
    // df(&d[0].1, srgba(1.0, 0.0, 0.0, 1.0));
    // df(&d[1].1, srgba(0.0, 0.0, 1.0, 1.0));

    let t = data::get_tmp();
    draw_points(&draw, &t[0], &t[1], srgba(1.0, 0.0, 0.0, 1.0), 3.0, s);
    draw_points(&draw, &t[2], &t[3], srgba(0.0, 0.0, 1.0, 1.0), 3.0, s);
    draw_points(&draw, &t[0], &t[4], srgba(1.0, 0.0, 1.0, 1.0), 3.0, s);
    draw_points(&draw, &t[4], &t[0], srgba(1.0, 0.0, 1.0, 1.0), 3.0, s);

    df(&d[3].1, srgba(1.0, 0.0, 0.0, 1.0));
    df(&d[4].1, srgba(0.0, 0.0, 1.0, 1.0));
    df(&d[5].1, srgba(1.0, 0.0, 1.0, 1.0));
    df(&d[2].1, srgba(0.0, 1.0, 0.0, 1.0));

    // draw_points(&draw, &vec![0.78], &vec![0.0], srgba(1.0, 1.0, 1.0, 1.0), 5.0, s);
    draw_root(&draw, &win, s, 0.78);

    // Draw the polyline as a stroked path.
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_points(draw: &Draw, xs: &Vec<f64>, ys: &Vec<f64>, color: Srgba, weight: f32, scale: f32) {
    for i in 0..xs.len() {
        draw.ellipse().color(color).w(weight).h(weight).x_y(
            scale * xs[i].to_f32().unwrap(),
            scale * ys[i].to_f32().unwrap(),
        );
    }
}

fn draw_func(
    draw: &Draw,
    func: &MyFunc,
    n_points: usize,
    scale: f32,
    left_x: f32,
    right_x: f32,
    color: Srgba,
) {
    let weight = 1.0;
    let y = |x: f32| func(x.to_f64().unwrap()).to_f32().unwrap();

    let vertices = (0..n_points).map(|i| {
        let x = map_range(i, 0, n_points - 1, left_x, right_x);
        pt2(scale * x, scale * y(x))
    });

    draw.polyline()
        .weight(weight)
        .join_round()
        .points(vertices)
        .color(color);
}

fn draw_grid(draw: &Draw, win: &Rect, s: f32) {
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
}

fn draw_root(draw: &Draw, win: &Rect, s: f32, x: f32) {
    draw.line()
        .start(pt2(s * x, win.top()))
        .end(pt2(s * x, win.bottom()))
        .weight(1.0)
        .color(GREY);

    draw.text(&(x).to_string()).x_y(s * x, -5.0);
}
