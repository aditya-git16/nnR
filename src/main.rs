use nn_r::model::prediction;
use nn_r::util::linear;

fn main() {
    let x = 4;
    let prediction = prediction(x);
    println!("{}" , prediction);
    let actual_value = linear(x);
    println!("{}" , actual_value);
    let error = prediction - actual_value;
    let error_absolute  = (prediction - actual_value).abs();
    println!("error  = {} , error_absolute = {}" , error , error_absolute);
}
