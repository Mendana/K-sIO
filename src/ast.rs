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
    Asin,
    Acos,
    Atan,
    Log,
    Ln,
    Exp,
    Sqrt,
    Abs,
    Floor,
    Ceil,
    Round,
}

impl Function {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sin" => Some(Function::Sin),
            "cos" => Some(Function::Cos),
            "tan" => Some(Function::Tan),
            "asin" => Some(Function::Asin),
            "acos" => Some(Function::Acos),
            "atan" => Some(Function::Atan),
            "log" => Some(Function::Log),
            "ln" => Some(Function::Ln),
            "exp" => Some(Function::Exp),
            "sqrt" => Some(Function::Sqrt),
            "abs" => Some(Function::Abs),
            "floor" => Some(Function::Floor),
            "ceil" => Some(Function::Ceil),
            "round" => Some(Function::Round),
            _ => None,
        }
    }
}