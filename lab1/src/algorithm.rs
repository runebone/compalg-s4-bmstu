pub fn get_newton_interpolation_func(xs: &Vec<f64>, ys: &Vec<f64>, x: f64, n: usize) -> Box<dyn Fn(f64) -> f64> {
    let x_nearest_index = find_index_of_nearest_x(&xs, x);
    let (start, end) = choose_n_nearest_points(n + 1, xs.len(), x_nearest_index);

    // Vector:
    // [0] -> P(x0) = y0
    // [1] -> P(x0, x1)
    // [2] -> P(x0, x1, x2)
    // ...
    // [n - 1] -> P(x0, x1, ..., x{n - 1})
    let dds: Vec<f64> = newton_calculate_dds(&xs, &ys, start, end);

    // dbg!(&dds);
    // dbg!(&x_nearest_index, &start, &end);

    interpolate_helper(xs[start..=end].to_vec(), dds)
}

pub fn newton_calculate_dds(xs: &Vec<f64>, ys: &Vec<f64>, start: usize, end: usize) -> Vec<f64> {
    // Vector of vectors, each containing values for DDs of corresponding order
    // i.e. vector vddv[0] contains values of DDs of 0th order (y-values)
    let mut vddv: Vec<Vec<f64>> = Vec::new();
    let n = end - start;
    let mut k = 0;

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

pub fn get_hermite_interpolation_func(xs: &Vec<f64>, ys: &Vec<f64>, dydxs: &Vec<f64>, x: f64, n: usize) -> Box<dyn Fn(f64) -> f64> {
    // FIXME: DRY
    let x_nearest_index = find_index_of_nearest_x(&xs, x);
    let (start, end) = choose_n_nearest_points(n / 2 + 1, xs.len(), x_nearest_index);
    let (xs_new, _) = hermite_transform_table_xy(xs, ys, start, end);

    let dds: Vec<f64> = hermite_calculate_dds(&xs, &ys, &dydxs, start, end);

    // dbg!(&dds);

    interpolate_helper(xs_new[0..xs_new.len()].to_vec(), dds)
}

pub fn hermite_calculate_dds(xs: &Vec<f64>, ys: &Vec<f64>, dydxs: &Vec<f64>, start: usize, end: usize) -> Vec<f64> {
    // FIXME: DRY
    // TODO: create func hermite_init(...) to take care of diff 1

    // Diff 1:
    let (xs_new, ys_new) = hermite_transform_table_xy(xs, ys, start, end);
    let n = xs_new.len() - 1;

    let mut vddv: Vec<Vec<f64>> = Vec::new();
    let mut k = 0;

    vddv.push(ys_new[..=n].to_vec());
    // ---

    for _ in 0..n {
        vddv.push(Vec::new()); // Reserve space for new dd-vector

        for i in 0..n - k {
            // TODO: maybe create a func which takes vddv, xs, dydxs k, i as
            // an arg and returns dd: hermite_calculate_dd(...)

            // Diff 2:
            let numerator = vddv[k][i] - vddv[k][i + 1];
            let denominator = xs_new[i] - xs_new[i + k + 1];

            let dd;
            if numerator == 0.0 && denominator == 0.0 {
                dd = dydxs[start + i / 2];
            } else {
                assert_ne!(denominator, 0.0);
                dd = numerator / denominator;
            }
            // ---

            vddv[k + 1].push(dd);
        }

        k += 1;
    }

    let mut dds_vec: Vec<f64> = Vec::new();

    for v in vddv {
        if v.len() > 0 {
            dds_vec.push(v[0]);
        }
    }

    dds_vec
}

fn hermite_transform_table_xy(xs: &Vec<f64>, ys: &Vec<f64>, start: usize, end: usize) -> (Vec<f64>, Vec<f64>) {
    let mut xs_new: Vec<f64> = Vec::new();
    let mut ys_new: Vec<f64> = Vec::new();

    for i in start..=end {
        xs_new.push(xs[i]);
        xs_new.push(xs[i]);

        ys_new.push(ys[i]);
        ys_new.push(ys[i]);
    }

    (xs_new, ys_new)
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

pub fn choose_n_nearest_points(n: usize, total_points: usize, current_point_index: usize) -> (usize, usize) {
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

pub fn find_index_of_nearest_x(xs: &Vec<f64>, x: f64) -> usize {
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
