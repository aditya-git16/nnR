// Define loss function

use crate::model::Model;

// square the loss function
pub fn loss_sq(errors: &Vec<f64> , model : Model) -> Vec<f64> {
    let mut loss = Vec::new();
    for i in 0..model.neurons{
        loss.push(errors[i] * errors[i]);
    }
    return loss;
}
