// define an epoch for a complete end to end run
// initial w -> initial prediction -> error -> loss -> gradient -> update w -> repeat

use crate::loss::loss_sq;
use crate::model::{Model};
use crate::util::linear;

pub fn epoch(train : &[f64] , model : &mut Model) {
    for i in train {
        let prediction = model.prediction(*i);
        println!("predicted value = {}" , prediction);
        let actual_value = linear(*i);
        println!("actual value = {}" , actual_value);
        let error = prediction - actual_value;
        println!("error = {}" , error);
        let loss = loss_sq(error);
        println!("loss = {}" , loss);
        model.update(actual_value, *i);
        println!("model weight after gradient descent = {}" , model.w)
    }
}