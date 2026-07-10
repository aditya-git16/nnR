use crate::{model::Model, types::Sample, util::{mv_linear, mv_linear_n}};

pub fn errors(input : Sample , prediction : Vec<f64> , model : Model) -> Vec<f64>{
    let mut errors = Vec::new();
    for i in 0..model.neurons{
        if i == 0 {
            let actual_value = mv_linear(input);
            let error = prediction[i] - actual_value;
            errors.push(error);
        }
        else{
            let actual_value = mv_linear_n(input);
            let error = prediction[i] - actual_value;
            errors.push(error);
        }
    }
    return errors;
}