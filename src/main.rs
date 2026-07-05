use nn_r::{epoch::epoch, model::Model, types::Train};

fn main() {
    // will use nested lists for mutiple inputs as a single training input in a training set
    let train : Train = [[1.1 , 2.3] , [2.0 , 2.9] , [3.2 , 9.1] , [4.0 , 5.7] , [5.0 , 2.4]];
    let n_epochs = 20;
    let mut runs = 0;
    let mut model = Model::new();
    println!("initial weight = {}" , model.w);
    while runs < n_epochs {
        epoch(&train , &mut model);
        runs += 1;
    }
    println!("final weight = {}" , model.w);
    println!("final bias = {}" , model.b);
}
