// making a basic model
// Initially only taking a single parameter w

use crate::types::Sample;

pub struct Model {
    pub weights: Vec<f64>,
    pub b: f64,
}

impl Model {
    pub fn new(n_weights: u64) -> Self {
        // initialise all wights to 0
        let mut weights = Vec::new();
        let mut i = 0;
        while i < n_weights {
            weights.push(0.0);
            i += 1;
        }
        Model {
            weights: weights,
            b: 0.0,
        }
    }

    pub fn update(&mut self, input: Sample , error : f64) {
        // learning rate
        let l_rate = 0.15;

        for (i, val) in input.iter().enumerate() {
            let w_new = self.weights[i] - l_rate * (2.0 * (error) * val);

            let b_new = self.b - l_rate * (2.0 * (error));

            self.weights[i] = w_new;
            self.b = b_new;
        }
    }

    pub fn prediction(&self, input: Sample) -> f64 {
        let mut value: f64 = 0.0;
        for (i, val) in input.iter().enumerate() {
            value = value + self.weights[i] * val
        }
        return value + self.b;
    }
}
