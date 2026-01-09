/// Module for mathematical functions.  

use crate::error::EvalError;

pub fn factorial(n: f64) -> Result<f64, EvalError> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err(EvalError::MathError("Factorial requires non-negative integers".to_string()));
    }

    let n = n as u64;
    if n > 20 {
        return Err(EvalError::MathError("Factorial too large".to_string()));
    }

    Ok((1..=n).product::<u64>() as f64)
}