// defining the structure of matrix

#[derive(Debug , Clone)]
pub struct Matrix {
    row : usize , 
    columns : usize , 
    data : Vec<f64>
}