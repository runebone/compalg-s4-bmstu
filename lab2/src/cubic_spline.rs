use crate::point::Points;

pub struct CubicSpline<'a> {
    pub data: &'a Points<f64>,
    pub coefficients: Vec<Coefficients>,
    l_edge_c: f64,
    r_edge_c: f64,
    edges_strategy: Box<dyn EdgesStrategy + 'a>,
}

impl<'a> CubicSpline<'a> {
    pub fn new(data: &'a Points<f64>) -> CubicSpline<'a> {
        CubicSpline {
            data,
            coefficients: Vec::new(),
            l_edge_c: 0.0,
            r_edge_c: 0.0,
            edges_strategy: Box::new(NaturalEdges),
        }
    }

    pub fn set_edges_strategy(&mut self, strategy: Box<dyn EdgesStrategy + 'a>) {
        self.edges_strategy = strategy;
    }

    pub fn compute(&mut self) {
        self.update_edges();
        self.coefficients.clear();

        let mut h: Vec<f64> = Vec::new();

        calculate_interval_lengths(&mut h, &self.data);

        let mut ksi: Vec<f64> = Vec::new();
        let mut eta: Vec<f64> = Vec::new();

        ksi.push(0.0);
        eta.push(self.l_edge_c / 2.0);

        calculate_run_coefficients(&mut ksi, &mut eta, &h, &self.data);

        let n = self.data.len();
        let mut c: Vec<f64> = vec![0.0; n];

        c[0] = self.l_edge_c / 2.0;
        c[n - 1] = self.r_edge_c / 2.0;

        calculate_c_coefficients(&mut c, &ksi, &eta);

        for i in 1..n {
            self.coefficients.push(Coefficients {
                a: self.data[i - 1].y,
                b: (self.data[i].y - self.data[i - 1].y) / h[i - 1] - 1.0 / 3.0 * h[i - 1] * (c[i] + 2.0 * c[i - 1]),
                c: c[i],
                d: (c[i] - c[i - 1]) / (3.0 * h[i - 1]),
            });
        }
    }

    fn update_edges(&mut self) {
        let (l_edge, r_edge) = self.edges_strategy.get_edges(&self.data);

        self.l_edge_c = l_edge;
        self.r_edge_c = r_edge;
    }

    pub fn func(&self) -> Box<dyn Fn(f64) -> f64 + '_> {
        Box::new(move |x| {
            let mut i = 0;

            while self.data[i + 1].x < x {
                i += 1;
            }

            let a = self.coefficients[i].a;
            let b = self.coefficients[i].b;
            let c = self.coefficients[i].c;
            let d = self.coefficients[i].d;
            let x0 = self.data[i].x;

            a + b * (x - x0) + c * (x - x0).powf(2.0) + d * (x - x0).powf(3.0)
        })
    }
}

fn calculate_c_coefficients(c: &mut Vec<f64>, ksi: &Vec<f64>, eta: &Vec<f64>) {
    for i in ksi.len() - 1..0 {
        c[i] = ksi[i] * c[i + 1] + eta[i];
    }
}

fn calculate_run_coefficients(ksi: &mut Vec<f64>, eta: &mut Vec<f64>, h: &Vec<f64>, data: &Points<f64>) {
    assert_eq!(ksi.len(), 1);
    assert_eq!(eta.len(), 1);

    for i in 1..h.len() {
        let a = h[i - 1];
        let b = -2.0 * (h[i - 1] + h[i]);
        let d = h[i];
        let f = -3.0 * ((data[i + 1].y - data[i].y) / h[i] - (data[i].y - data[i - 1].y) / h[i - 1]);

        ksi.push(d / (b - a * ksi[i - 1]));
        eta.push((a * eta[i - 1] + f) / (b - a * ksi[i - 1]));
    }
}

fn calculate_interval_lengths(dxs: &mut Vec<f64>, data: &Points<f64>) {
    assert_eq!(dxs.len(), 0);

    let mut xi_prev = data[0].x;

    for i in 1..data.len() {
        dxs.push(data[i].x - xi_prev);

        xi_prev = data[i].x;
    }
}

trait EdgesStrategy {
    fn get_edges(&self, data: &Points<f64>) -> (f64, f64);
}

struct NaturalEdges;

impl EdgesStrategy for NaturalEdges {
    fn get_edges(&self, _data: &Points<f64>) -> (f64, f64) {
        (0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Coefficients {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
