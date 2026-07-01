use nn_r::{epoch::epoch, model::Model};

fn main() {
    let train : [f64 ; 5] = [1.1 , 2.0 , 3.2 , 4.0, 5.0];
    let n_epochs = 10;
    let mut runs = 0;
    let mut model = Model::new();
    println!("initial weight = {}" , model.w);
    while runs < n_epochs {
        epoch(&train , &mut model);
        runs += 1;
    }
    println!("final weight = {}" , model.w);
}
