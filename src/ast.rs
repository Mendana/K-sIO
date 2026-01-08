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
    PostfixOp {
        expr: Box<Expr>,
        op: UnOp,
    },
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

impl Function {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sin" => Some(Function::Sin),
            "cos" => Some(Function::Cos),
            "tan" => Some(Function::Tan),
            "log" => Some(Function::Log),
            "ln" => Some(Function::Ln),
            "exp" => Some(Function::Exp),
            "sqrt" => Some(Function::Sqrt),
            "abs" => Some(Function::Abs),
            _ => None,
        }
    }
}