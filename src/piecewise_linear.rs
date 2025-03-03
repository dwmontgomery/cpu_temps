#[derive(Debug)]
pub struct Points {
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub b_values: Vec<f64>,
    pub m_values: Vec<f64>,
    pub y_calc_values: Vec<f64>,
}

impl Points {
    pub fn new(x_val: Vec<f64>, y_val: Vec<f64>) -> Self {
        assert!(x_val.len() == y_val.len(), "x_values and y_values must be the same length");
        Points{x_values: x_val, 
                y_values: y_val,
                b_values: Vec::new(),
                m_values: Vec::new(),
                y_calc_values: Vec::new(),
        }
    }

    pub fn solve_m(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
        (y1-y0) / (x1-x0)
    }

    pub fn solve_b(x0: f64, y0: f64, m: f64) -> f64 {
        y0 - m * x0
    }

    pub fn solve_y(b: f64, m: f64, x: f64) -> f64 {
        b + m * x
    }

    pub fn interpolate(&mut self) {
        let len = self.x_values.len()-1;
        
        self.b_values = vec![0.0; len];
        self.m_values = vec![0.0; len];
        self.y_calc_values = vec![0.0; len];
        
        for k in 0..len {
            let x_k = self.x_values[k];
            let x_k1 = self.x_values[k + 1];
            let y_k = self.y_values[k];
            let y_k1 = self.y_values[k + 1];

                // Calculate the slope (m) and the intercept (b)
            self.m_values[k] = Points::solve_m(x_k, y_k, x_k1, y_k1);
            self.b_values[k] = Points::solve_b(x_k, y_k, self.m_values[k]);
            self.y_calc_values[k] = Points::solve_y(self.b_values[k], self.m_values[k], x_k);
        }
    }

    pub fn print(&self) {
        for k in 0..self.x_values.len()-1 {
            println!("{:6} <= x < {:6} ; y = {:20.08} + {:12.08} x ; interpolation", self.x_values[k], self.x_values[k+1], self.b_values[k], self.m_values[k]);
        }
        
    }
}

