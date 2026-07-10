// define an epoch for a complete end to end run
// initial w -> initial prediction -> error -> loss -> gradient -> update w -> repeat

use crate::error::errors;
use crate::loss::loss_sq;
use crate::model::Model;
use crate::types::Train;

pub fn epoch(train: &Train, model: &mut Model) {
    for i in train {
        let prediction = model.prediction(*i);
        let errors = errors(*i, prediction, model.clone());
        println!("error = {:?}", errors);
        let loss = loss_sq(&errors , model.clone());
        println!("loss = {:?}", loss);
        model.update(*i, &errors);
        println!(
            "model weight after gradient descent = {:?} , bias = {:?}",
            model.weights, model.b
        )
    }
}
