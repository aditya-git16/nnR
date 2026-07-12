// defining the structure of matrix

#[derive(Debug , Clone)]
pub struct Matrix {
    pub rows : usize , 
    pub columns : usize , 
    pub data : Vec<f64>
}

impl Matrix {
    // we can define multiple ways to initialise a matrix
    // first can we be that we pass the size  (row , column ) and create 
    // a matirx with 0 values
    // second we can also pass a 2D array to initialise a matrix -> we will 
    // convert this into matrix with data in 1D array
    // third we can pass a 1D array with size and create the matrix struct accordingly
    pub fn new(row : usize , column : usize) -> Self {
        Matrix { rows : row, columns : column , data: vec![0.0 ; row*column] }
    }

    pub fn new_from_1d(row : usize , column : usize , data : Vec<f64>) -> Self {
        Matrix {rows : row , columns : column , data : data}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_initialisation(){
        let matrix = Matrix::new(2, 2);
        println!("Matrix = {:?}" , matrix);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.columns, 2);
    }

    #[test]
    fn test_matrix_initialisation_1d() {
        let data = vec![3.0 , 2.6 , 7.1 , 9.5 ];
        let matrix = Matrix::new_from_1d(2, 2, data.clone());
        println!("Matrix = {:?}" , matrix);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.columns, 2);
        assert_eq!(matrix.data , data);
    }
}