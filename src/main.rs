use nn_r::model::prediction;
use nn_r::util::linear;
use nn_r::loss::loss_sq;

fn main() {
    let x = 4.0;
    let prediction = prediction(x);
    println!("prediction = {}" , prediction);
    let actual_value = linear(x);
    println!("actual_value = {}" , actual_value);
    let error = prediction - actual_value;
    let error_absolute  = (prediction - actual_value).abs();
    println!("error  = {} , error_absolute = {}" , error , error_absolute);
    let loss = loss_sq(error);
    println!("Loss = {}", loss)
}
