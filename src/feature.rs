use crate::types::Train;

// implementing min max normalisation
pub fn normalise(input: &Train) -> Train {
    let n_features = input[0].len();
    let mut output = *input;
    for col in 0..n_features {
        let column: Vec<f64> = input.iter().map(|row| row[col]).collect();

        let min = column.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = column.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        for row in 0..input.len() {
            output[row][col] = (input[row][col] - min) / (max - min);
        }
    }
    return output;
}
