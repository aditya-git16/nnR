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

    pub fn new_from_2d(input : Vec<Vec<f64>>) -> Self {
        let mut rows = 0;
        let columns = input[0].len();
        let mut data = Vec::new();
        for i in input {
            for j in i {
                data.push(j);
            }
            rows += 1;
        }
        Matrix { rows: rows, columns, data }
    }

    // return the shape of the matrix
    pub fn shape(&self) -> (usize , usize) {
        return (self.rows , self.columns);
    }

    pub fn set_by_index(&mut self , row : usize , column : usize , value : f64){
        self.data[row*self.columns + column] = value;
    }

    pub fn get_by_index(&self , row : usize , column : usize) -> f64 {
        return self.data[row*self.columns + column]
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

    #[test]
    fn test_matrix_initialisation_2d() {
        let data = vec![vec![3.0 , 2.6 ], vec![7.1 , 9.5] ];
        let matrix = Matrix::new_from_2d(data.clone());
        println!("Matrix = {:?}" , matrix);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.columns, 2);
        let expected: Vec<f64> = data.into_iter().flatten().collect();
        assert_eq!(matrix.data, expected);
    }

    #[test]
    pub fn test_shape(){
        let matrix = Matrix::new(2, 2);
        let shape = matrix.shape();
        assert_eq!(matrix.rows , shape.0);
        assert_eq!(matrix.columns , shape.1);
    }
}