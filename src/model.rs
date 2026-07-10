// making a basic model
// Initially only taking a single parameter w

use crate::types::Sample;

// 2 neurons for 2 outputs
// each neuron will have its own vector of weights
// weights : [neuron_1 : [w1 , w2 , w3 , ....] , neuron_2 : [w1 , w2 , w3 , ....] , ...]
#[derive(Debug, Clone)]
pub struct Model {
    pub weights: Vec<Vec<f64>>,
    pub b: Vec<f64>,
    pub neurons: usize,
}

impl Model {
    pub fn new(n_neurons : usize , n_weights: usize) -> Self {
        // initial weight vector which will contian weights of all neurons
        let mut weights = Vec::new();
        // loop for intialising neuron weight vectors
        for _ in 0..n_neurons {
            // push the neuron vector in the main weights vector
            weights.push(vec![0.0 ; n_weights]);
        }
        Model {
            weights: weights,
            b: vec![0.0 ; n_neurons],
            neurons : n_neurons
        }
    }

    pub fn update(&mut self, input: Sample, errors: &Vec<f64>) {
        // learning rate
        let l_rate = 0.15;

        for j in 0..self.neurons{
            for (i, val) in input.iter().enumerate() {
                let w_new = self.weights[j][i] - l_rate * (2.0 * (errors[j]) * val);
    
                let b_new = self.b[j] - l_rate * (2.0 * (errors[j]));
    
                self.weights[j][i] = w_new;
                self.b[j] = b_new;
            }
        }
    }

    pub fn prediction(&self, input: Sample) -> Vec<f64> {
        let mut values: Vec<f64> = vec![0.0; self.neurons];
        for j in 0..self.neurons{
            for (i, val) in input.iter().enumerate() {
                values[j] = values[j] + self.weights[j][i] * val
            }
            values[j] = values[j] + self.b[j];
        }
        return values;
    }
}
