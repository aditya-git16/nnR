// making a basic model
// Initially only taking a single parameter w

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

    pub fn update(&mut self, actual_value: f64, input: f64) {
        // learning rate
        let l_rate = 0.1;

        let w_new = self.w - l_rate * (2.0 * (self.w * input + self.b - actual_value) * input);

        let b_new = self.b - l_rate * (2.0 * (self.w * input + self.b - actual_value));

        self.w = w_new;
        self.b = b_new;
    }

    pub fn prediction(&self, input: f64) -> f64 {
        return self.w * input + self.b;
    }
}
