/// Abastract Syntax Tree (AST) module for the calculator language.

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    FunctionCall {
        func: Function,
        args: Vec<Expr>,
    },
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Negate,
    Positive,
    Factorial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Exp,
    Sqrt,
    Abs,
}