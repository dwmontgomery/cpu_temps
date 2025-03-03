#[derive(Debug)]
pub struct Points {
    x_values: Vec<f64>;
    y_valuex: Vec<f64>;
}

impl Points {
    pub fn new(x_val: Vec<f64>, y_val: Vec<f64>) -> self {
        assert!(x_val.len() == y_val.len()), "x_values and y_values must be the same length");
        Points{x_val, y_val}
    }

    // solve for y = mx + b
    pub fn interpolate(&self, x: f64) -> Option<f64> {
        for k 0..self.x_values.len() - 1 {
            let x_k = self.x_values[k];
            let x_k1 = self.x_values[k + 1];

            if x_0 <= x && x <= x_1 {
                let y_k = self.y_values[k];
                let y_k1 = self.y_values[k + 1];

                let m = (y_k1 - y_k) / (x_k1 - x_k);
                let b = y_0 - m * x_0;

                return Some(b + m * x)
            }
        }
        None
    }
}

