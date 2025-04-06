#[derive(Debug)]
pub struct Points {
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub b_values: Vec<f64>,
    pub m_values: Vec<f64>,
    pub y_calc_values: Vec<f64>,
    pub least_sq_b: f64,
    pub least_sq_m: f64,
}
// x_values are the time values for the input
// y_values are the temperature values read from the input
// b_values are calculated
// m_values are calculated
// y_cal_values are calculated
impl Points {
    pub fn new(x_val: Vec<f64>, y_val: Vec<f64>) -> Self {
        assert!(x_val.len() == y_val.len(), "x_values and y_values must be the same length");
        Points{x_values: x_val, 
                y_values: y_val,
                b_values: Vec::new(),
                m_values: Vec::new(),
                y_calc_values: Vec::new(),
                least_sq_b: 0.0,
                least_sq_m: 0.0,
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

    pub fn least_squares(&mut self) {
        let len = self.x_values.len();

        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut f: f64 = 0.0;
        let mut g: f64 = 0.0;

        for k in 0..len {
            a = a + self.x_values[k];
            b = b + self.x_values[k] * self.x_values[k];
            f = f + self.y_values[k];
            g += self.x_values[k] * self.y_values[k];
        }
        let n = len as f64;
        // println!("a is {} \nb is {} \nf is {} \ng is {} \nn is {}", a, b, f, g, n);
        let c_1 = (n * g - a * f) / (n * b - a.powi(2));
        let c_0 = f / n - a / n * c_1;
        // println!("c_0 is {} \nc_1 is {}", c_0, c_1);
        self.least_sq_b = c_0;
        self.least_sq_m = c_1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_m() {
        let m = Points::solve_m(1.0, 2.0, 3.0, 6.0);
        assert_eq!(m, 2.0);
    }

    #[test]
    fn test_solve_b() {
        let b = Points::solve_b(2.0, 9.0, 4.0);
        assert_eq!(b, 1.0);
    }

    #[test]
    fn test_solve_y() {
        let y = Points::solve_y(2.0, 1.0, 4.0);
        assert_eq!(y, 6.0);
    }

    #[test]
    fn test_interpolate() {
        let test_values: Vec<f64> = vec![20.0, 40.0, 30.0];
        let test_times: Vec<f64> = vec![0.0, 30.0, 60.0];
        let mut test_points = Points::new(test_times, test_values);
        test_points.least_squares();
        
    }
}

