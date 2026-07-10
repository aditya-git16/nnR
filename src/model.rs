// making a basic model
// Initially only taking a single parameter w

use crate::types::Sample;

// 2 neurons for 2 outputs
// each neuron will have its own vector of weights
// weights : [neuron_1 : [w1 , w2 , w3 , ....] , neuron_2 : [w1 , w2 , w3 , ....] , ...]
pub struct Model {
    pub weights: Vec<Vec<f64>>,
    pub b: f64,
}

impl Model {
    pub fn new(n_neurons : u64 , n_weights: u64) -> Self {
        // initial weight vector which will contian weights of all neurons
        let mut weights = Vec::new();
        // loop for intialising neuron weight vectors
        for _ in 0..n_neurons {
            // create empty neuron_weight vector
            let mut neuron_weights = Vec::new();
            // n_weights = _n_features , so push n_weights in the neuron weight vetor
            for _ in 0..n_weights{
                neuron_weights.push(0.0);
            }
            // push the neuron vector in the main weights vector
            weights.push(neuron_weights);
        }
        Model {
            weights: weights,
            b: 0.0,
        }
    }

    pub fn update(&mut self, input: Sample, error: f64) {
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
