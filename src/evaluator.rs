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

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<f64, EvalError> {
        let result = self.eval(expr)?;
        self.context.set_ans(result);
        Ok(result)
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
                self.eval_function(func, &arg_values)
            },

            Expr::Assignment { name, value } => {
                let result = self.eval(value)?;
                self.context.set(name.clone(), result);
                Ok(result)
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

    fn eval_function(&self, func: &Function, args: &[f64]) -> Result<f64, EvalError> {
        match func {
            Function::Sin => {
                Self::validate_args(args, 1, "sin")?;
                let radians = functions::to_radians(
                    args[0],
                    self.get_context().get_angle_mode()
                );
                Ok(radians.sin())
            },
            Function::Cos => {
                Self::validate_args(args, 1, "cos")?;
                let radians = functions::to_radians(
                    args[0],
                    self.get_context().get_angle_mode()
                );
                Ok(radians.cos())
            },
            Function::Tan => {
                Self::validate_args(args, 1, "tan")?;
                let radians = functions::to_radians(
                    args[0],
                    self.get_context().get_angle_mode()
                );
                Ok(radians.tan())
            },
            Function::Asin => {
                Self::validate_args(args, 1, "asin")?;
                if args[0] < -1.0 || args[0] > 1.0 {
                    return Err(EvalError::MathError("asin domain error: input must be in [-1, 1]".to_string()));
                }
                let radians = args[0].asin();
                Ok(functions::from_radians(
                    radians,
                    self.get_context().get_angle_mode()
                ))
            },
            Function::Acos => {
                Self::validate_args(args, 1, "acos")?;
                if args[0] < -1.0 || args[0] > 1.0 {
                    return Err(EvalError::MathError("acos domain error: input must be in [-1, 1]".to_string()));
                }
                let radians = args[0].acos();
                Ok(functions::from_radians(
                    radians,
                    self.get_context().get_angle_mode()
                ))
            },
            Function::Atan => {
                Self::validate_args(args, 1, "atan")?;
                let radians = args[0].atan();
                Ok(functions::from_radians(
                    radians,
                    self.get_context().get_angle_mode()
                ))
            },
            Function::Ln => {
                Self::validate_args(args, 1, "ln")?;
                if args[0] <= 0.0 {
                    return Err(EvalError::MathError("ln of non-positive number".to_string()));
                }
                Ok(args[0].ln())
            },
            Function::Log => {
                Self::validate_args(args, 1, "log")?;
                if args[0] <= 0.0 {
                    return Err(EvalError::MathError("log of non-positive number".to_string()));
                }
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
            Function::Ceil => {
                Self::validate_args(args, 1, "ceil")?;
                Ok(args[0].ceil())
            },
            Function::Floor => {
                Self::validate_args(args, 1, "floor")?;
                Ok(args[0].floor())
            },
            Function::Round => {
                Self::validate_args(args, 1, "round")?;
                Ok(args[0].round())
            },
            Function::Pow => {
                Self::validate_args(args, 2, "pow")?;
                Ok(args[0].powf(args[1]))
            },
            Function::Max => {
                if args.is_empty() {
                    return Err(EvalError::InvalidArguments("max expects at least 1 argument".to_string()));
                }
                Ok(args.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
            },
            Function::Min => {
                if args.is_empty() {
                    return Err(EvalError::InvalidArguments("min expects at least 1 argument".to_string()));
                }
                Ok(args.iter().cloned().fold(f64::INFINITY, f64::min))
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