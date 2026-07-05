use nn_r::{epoch::epoch, model::Model, types::Train};

fn main() {
    // will use nested lists for mutiple inputs as a single training input in a training set
    let train: Train = [[1.1, 2.3], [2.0, 2.9], [3.2, 9.1], [4.0, 5.7], [5.0, 2.4]];
    let n_epochs = 20;
    let mut runs = 0;
    // for now lets assume number if weights = number of values in a sample set
    let sample_len: u64 = train[0].len() as u64;
    let mut model = Model::new(sample_len);
    println!("initial weight = {}", model.w);
    while runs < n_epochs {
        epoch(&train, &mut model);
        runs += 1;
    }
    println!("final weight = {}", model.w);
    println!("final bias = {}", model.b);
}
