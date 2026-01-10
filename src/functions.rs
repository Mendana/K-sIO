/// Module for mathematical functions.  

use crate::{context::AngleMode, error::EvalError};

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

pub fn to_radians(angle: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Degrees => angle.to_radians(),
        AngleMode::Radians => angle,
        AngleMode::Gradians => angle * std::f64::consts::PI / 200.0,
    }
}

pub fn from_radians(angle: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Degrees => angle.to_degrees(),
        AngleMode::Radians => angle,
        AngleMode::Gradians => angle * 200.0 / std::f64::consts::PI,
    }
}