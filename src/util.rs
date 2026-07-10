use crate::types::Sample;

// defining a linear reltionship for testing

pub fn linear(x: f64) -> f64 {
    let c = 3.0;
    return 2.0 * x + c;
}

// define multi variable linear function
pub fn mv_linear(sample: Sample) -> f64 {
    let constant = 12.0;
    return 3.2 * sample[0] + 1.3 * sample[1] + constant;
}

// define new multi variable linear function for multiple output testing
pub fn mv_linear_n(sample: Sample) -> f64 {
    let constant = 13.2;
    return 1.6 * sample[0] + 2.8 * sample[1] + constant;
}