/// Evaluator module for processing expressions.

use crate::{ast::{BinOp, Expr, Function, UnOp}, context::Context, error::EvalError, functions};

pub struct Evaluator {
    context: Context,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            context: Context::new()
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<f64, EvalError> {
        match expr {
            Expr::Number(n) => Ok(*n),

            Expr::Variable(name) => {
                self.context.get(name)
                    .ok_or_else(|| EvalError::UndefinedVariable(name.clone()))
            },

            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                Self::eval_binary_op(op, left_val, right_val)
            },

            Expr::UnaryOp { op, expr } => {
                let val = self.eval(expr)?;
                Self::eval_unary_op(op, val)
            },

            Expr::PostfixOp { expr, op } => {
                let val = self.eval(expr)?;
                Self::eval_unary_op(op, val)
            },

            Expr::FunctionCall { func, args } => {
                let arg_values: Result<Vec<f64>, _> = args.iter().map(|arg| self.eval(arg)).collect();

                let arg_values = arg_values?;
                Self::eval_function(func, &arg_values)
            }
        }
    }

    fn eval_binary_op(op: &BinOp, left: f64, right: f64) -> Result<f64, EvalError> {
        match op {
            BinOp::Add => Ok(left + right),
            BinOp::Subtract => Ok(left - right),
            BinOp::Multiply => Ok(left * right),
            BinOp::Divide => {
                if right == 0.0 {
                    Err(EvalError::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            },
            BinOp::Power => Ok(left.powf(right)),
        }
    }

    fn eval_unary_op(op: &UnOp, val: f64) -> Result<f64, EvalError> {
        match op {
            UnOp::Positive => Ok(val),
            UnOp::Negate => Ok(-val),
            UnOp::Factorial => functions::factorial(val),
        }
    }

    fn eval_function(func: &Function, args: &[f64]) -> Result<f64, EvalError> {
        match func {
            Function::Sin => {
                Self::validate_args(args, 1, "sin")?;
                Ok(args[0].to_radians().sin())
            },
            Function::Cos => {
                Self::validate_args(args, 1, "cos")?;
                Ok(args[0].to_radians().cos())
            },
            Function::Tan => {
                Self::validate_args(args, 1, "tan")?;
                Ok(args[0].to_radians().tan())
            },
            Function::Ln => {
                Self::validate_args(args, 1, "ln")?;
                Ok(args[0].ln())
            },
            Function::Log => {
                Self::validate_args(args, 1, "log")?;
                Ok(args[0].log10())
            },
            Function::Sqrt => {
                if args.len() != 1 {
                    return Err(EvalError::InvalidArguments("sqrt expects 1 argument".to_string()));
                }
                if args[0] < 0.0 {
                    return Err(EvalError::MathError("sqrt of negative number".to_string()));
                }
                Ok(args[0].sqrt())
            },
            Function::Abs => {
                Self::validate_args(args, 1, "abs")?;
                Ok(args[0].abs())
            },
            Function::Exp => {
                Self::validate_args(args, 1, "exp")?;
                Ok(args[0].exp())
            },
        }
    }

    fn validate_args(args: &[f64], expected: usize, func_name: &str) -> Result<(), EvalError> {
        if args.len() != expected {
            Err(EvalError::InvalidArguments(
                format!("{} expects {} argument(s), got {}", func_name, expected, args.len())
            ))
        } else {
            Ok(())
        }
    }
}