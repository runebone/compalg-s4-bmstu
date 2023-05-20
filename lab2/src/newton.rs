use crate::point::Points;

pub fn get_newton_interpolation_func(
    points: &Points<f64>,
    // xs: &Vec<f64>,
    // ys: &Vec<f64>,
    x: f64,
    n: usize,
) -> Box<dyn Fn(f64) -> f64> {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for i in 0..points.len() {
        xs.push(points[i].x);
        ys.push(points[i].y);
    }

    let x_nearest_index = find_index_of_nearest_x(&xs, x);
    let (start, end) = choose_n_nearest_points(n + 1, xs.len(), x_nearest_index);

    // Vector:
    // [0] -> P(x0) = y0
    // [1] -> P(x0, x1)
    // [2] -> P(x0, x1, x2)
    // ...
    // [n - 1] -> P(x0, x1, ..., x{n - 1})
    // let dds: Vec<f64> = newton_calculate_dds(&xs, &ys, start, end);
    let dds: Vec<f64> = newton_calculate_dds(&points, start, end);

    interpolate_helper(xs[start..=end].to_vec(), dds)
}

// pub fn newton_calculate_dds(xs: &Vec<f64>, ys: &Vec<f64>, start: usize, end: usize) -> Vec<f64> {
pub fn newton_calculate_dds(points: &Points<f64>, start: usize, end: usize) -> Vec<f64> {
    // Vector of vectors, each containing values for DDs of corresponding order
    // i.e. vector vddv[0] contains values of DDs of 0th order (y-values)
    let mut vddv: Vec<Vec<f64>> = Vec::new();
    let n = end - start;
    let mut k = 0;

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for i in 0..points.len() {
        xs.push(points[i].x);
        ys.push(points[i].y);
    }

    vddv.push(ys[start..=end].to_vec());

    // Calculate DDs and push them to corresponding vectors
    // (No recursion, dynamic programming)
    for _ in 0..n {
        vddv.push(Vec::new()); // Reserve space for new dd-vector

        for i in 0..n - k {
            let numerator = vddv[k][i] - vddv[k][i + 1];
            let denominator = xs[start + i] - xs[start + i + k + 1];

            assert_ne!(denominator, 0.0);
            let dd = numerator / denominator;

            vddv[k + 1].push(dd);
        }

        k += 1;
    }

    let mut dds_vec: Vec<f64> = Vec::new();

    // Choose only necessary values (P(x0), P(x0, x1), P(x0, x1, x2), ...)
    for v in vddv {
        if v.len() > 0 {
            dds_vec.push(v[0]);
        }
    }

    dds_vec
}

fn interpolate_helper(xs: Vec<f64>, dds: Vec<f64>) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        let mut result: f64 = 0.0;
        for i in 0..xs.len() {
            let mut term: f64 = 1.0;
            for j in 0..i {
                term *= x - xs[j];
            }
            result += term * dds[i];
        }
        result
    })
}

pub fn choose_n_nearest_points(
    n: usize,
    total_points: usize,
    current_point_index: usize,
) -> (usize, usize) {
    assert!(n <= total_points);
    assert!(current_point_index < total_points);

    let mut start: usize = current_point_index;
    let mut end: usize = current_point_index;

    // Start choosing from the next of current_point_index
    let mut i = 0;

    for _ in 1..n {
        if i % 2 == 0 {
            if end < total_points - 1 {
                end += 1;
            }
            if start > 0 {
                i += 1;
            }
        } else {
            if start > 0 {
                start -= 1;
            }
            if end < total_points - 1 {
                i += 1;
            }
        }
    }

    (start, end)
}

fn find_index_of_nearest_x(xs: &Vec<f64>, x: f64) -> usize {
    let mut i_nearest_x = 0;
    let mut min_diff = (x - xs[0]).abs();

    for i in 1..xs.len() {
        let diff = (x - xs[i]).abs();
        if diff < min_diff {
            i_nearest_x = i;
            min_diff = diff;
        }
    }

    i_nearest_x
}
