use crate::table::Table;

pub fn interpolate_newton(table: &Table, x: f64, n: usize) -> f64 {
    // FIXME: DRY
    let (start, _) = choose_n_points_from_table(table, x, n);

    // Vector:
    // [0] -> P(x0) = y0
    // [1] -> P(x0, x1)
    // [2] -> P(x0, x1, x2)
    // ...
    // [n - 1] -> P(x0, x1, ..., x{n - 1})
    let vec_of_dd_values: Vec<f64> = get_vec_of_dd_values(table, x, n);

    // Compute polynome based on calculated dd-s for given x
    
    let mut sum: f64 = 0.0;
    for i in 0..n + 1 {
        let mut term: f64 = 1.0;
        for j in start..start + i {
            term *= x - table.records[j].x;
        }
        sum += term * vec_of_dd_values[i];
    }

    sum
}

pub fn get_vec_of_dd_values(table: &Table, x: f64, n: usize) -> Vec<f64> {
    let (start, end) = choose_n_points_from_table(table, x, n);

    // vec of dd-vectors
    let mut vddv: Vec<Vec<f64>> = Vec::new();
    let mut s = 0;

    let mut x_vec: Vec<f64> = Vec::new();
    let mut y_vec: Vec<f64> = Vec::new();
    for i in start..=end {
        // FIXME: ugly; depends on table interface
        x_vec.push(table.records[i].x);
        y_vec.push(table.records[i].y);
    }
    vddv.push(y_vec);

    for _ in 0..n {
        let mut _dd_vec: Vec<f64> = Vec::new();

        for i in 0..n - s {
            let dd = (vddv[s][i] - vddv[s][i + 1]) / (x_vec[i] - x_vec[i + s + 1]);
            vddv.push(Vec::new());
            vddv[s + 1].push(dd);
        }

        vddv.push(_dd_vec);
        s += 1;
    }

    let mut vec_of_dd_values: Vec<f64> = Vec::new();

    for v in vddv {
        if v.len() > 0 {
            vec_of_dd_values.push(v[0]);
        }
    }

    vec_of_dd_values
}

pub fn choose_n_points_from_table(table: &Table, x: f64, n: usize) -> (usize, usize) {
    assert!(n <= table.records.len());

    let i_nearest_table_x = find_index_of_nearest_to_x_point_from_the_table(table, x);

    // Choose n nearest to nearest_table_x x-s from the table
    let table_len = table.records.len();
    let mut i_start: usize = i_nearest_table_x;
    let mut i_end: usize = i_nearest_table_x;
    let mut i = 0; // Start choosing from the next of i_nearest... index
    for _ in 0..n {
        if i % 2 == 0 {
            if i_end < table_len - 1 {
                i_end += 1;
            }
            if i_start > 0 {
                i += 1;
            }
        } else {
            if i_start > 0 {
                i_start -= 1;
            }
            if i_end < table_len - 1 {
                i += 1;
            }
        }
    }
    // dbg!(i_nearest_table_x);

    // n+1 nodes
    (i_start, i_end)
}

fn find_index_of_nearest_to_x_point_from_the_table(table: &Table, x: f64) -> usize {
    let mut i_nearest_table_x = 0;
    let mut diff = (x - table[0].x).abs();

    for i in 1..table.records.len() {
        let tmp = (x - table[i].x).abs();
        if tmp < diff {
            i_nearest_table_x = i;
            diff = tmp;
        }
    }

    i_nearest_table_x
}
