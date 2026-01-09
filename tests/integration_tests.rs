use matheval::{lexer::Lexer, parser::Parser, evaluator::Evaluator};

fn eval_expr(input: &str) -> Result<f64, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| format!("{}", e))?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("{}", e))?;
    
    let mut evaluator = Evaluator::new();
    evaluator.eval(&ast).map_err(|e| format!("{}", e))
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(eval_expr("2 + 3").unwrap(), 5.0);
    assert_eq!(eval_expr("10 - 4").unwrap(), 6.0);
    assert_eq!(eval_expr("3 * 4").unwrap(), 12.0);
    assert_eq!(eval_expr("15 / 3").unwrap(), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(eval_expr("2 + 3 * 4").unwrap(), 14.0);
    assert_eq!(eval_expr("(2 + 3) * 4").unwrap(), 20.0);
}

#[test]
fn test_power() {
    assert_eq!(eval_expr("2^3").unwrap(), 8.0);
    assert_eq!(eval_expr("2^3^2").unwrap(), 512.0); // 2^(3^2) = 2^9 = 512
}

#[test]
fn test_functions() {
    assert!((eval_expr("sqrt(16)").unwrap() - 4.0).abs() < 1e-10);
    assert!((eval_expr("abs(-5)").unwrap() - 5.0).abs() < 1e-10);
}

#[test]
fn test_factorial() {
    assert_eq!(eval_expr("5!").unwrap(), 120.0);
    assert_eq!(eval_expr("0!").unwrap(), 1.0);
}

#[test]
fn test_constants() {
    assert!((eval_expr("pi").unwrap() - std::f64::consts::PI).abs() < 1e-10);
    assert!((eval_expr("e").unwrap() - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_division_by_zero() {
    assert!(eval_expr("1 / 0").is_err());
}

#[test]
fn test_complex_expressions() {
    assert_eq!(eval_expr("2 + 3 * 4 - 5 / 2").unwrap(), 11.5);
    assert_eq!(eval_expr("((2 + 3) * 4) - (10 / 2)").unwrap(), 15.0);
    assert!((eval_expr("2 * pi").unwrap() - (2.0 * std::f64::consts::PI)).abs() < 1e-10);
}

#[test]
fn test_nested_parentheses() {
    assert_eq!(eval_expr("((2 + 3) * (4 + 5))").unwrap(), 45.0);
    assert_eq!(eval_expr("(((1 + 2) * 3) + 4)").unwrap(), 13.0);
}

#[test]
fn test_unary_minus() {
    assert_eq!(eval_expr("-5").unwrap(), -5.0);
    assert_eq!(eval_expr("-5 + 3").unwrap(), -2.0);
    assert_eq!(eval_expr("-(5 + 3)").unwrap(), -8.0);
}

#[test]
fn test_power_with_parentheses() {
    assert_eq!(eval_expr("(2 + 3)^2").unwrap(), 25.0);
    assert_eq!(eval_expr("2^(3 + 1)").unwrap(), 16.0);
}

#[test]
fn test_factorial_combinations() {
    assert_eq!(eval_expr("3! + 2!").unwrap(), 8.0);
    assert_eq!(eval_expr("(2 + 3)!").unwrap(), 120.0);
}

#[test]
fn test_multiple_functions() {
    assert!((eval_expr("sqrt(16) + abs(-3)").unwrap() - 7.0).abs() < 1e-10);
    assert!((eval_expr("sqrt(abs(-25))").unwrap() - 5.0).abs() < 1e-10);
}

#[test]
fn test_trig_functions() {
    assert!((eval_expr("sin(0)").unwrap() - 0.0).abs() < 1e-10);
    assert!((eval_expr("cos(0)").unwrap() - 1.0).abs() < 1e-10);
    assert!((eval_expr("tan(0)").unwrap() - 0.0).abs() < 1e-10);
}

#[test]
fn test_logarithms() {
    assert!((eval_expr("ln(e)").unwrap() - 1.0).abs() < 1e-10);
    assert!((eval_expr("log(10)").unwrap() - 1.0).abs() < 1e-10);
}

#[test]
fn test_edge_cases() {
    assert_eq!(eval_expr("0 * 999999").unwrap(), 0.0);
    assert_eq!(eval_expr("1^1000").unwrap(), 1.0);
    assert!((eval_expr("sqrt(0)").unwrap() - 0.0).abs() < 1e-10);
}

#[test]
fn test_decimal_numbers() {
    assert_eq!(eval_expr("3.5 + 2.5").unwrap(), 6.0);
    assert_eq!(eval_expr("0.1 + 0.2").unwrap(), 0.30000000000000004);
}

#[test]
fn test_negative_sqrt() {
    assert!(eval_expr("sqrt(-1)").is_err());
}

#[test]
fn test_negative_factorial() {
    assert!(eval_expr("(-5)!").is_err());
}