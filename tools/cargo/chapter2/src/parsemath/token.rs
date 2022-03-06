pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Number(f64),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidOperator(String),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Node {
    Num(f64),
    Add(Box<Node>, Box<Node>),
    Substract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
}

pub enum OperPrec {
    DefaultZero,
    Negative,
    Caret,
    MultiplyDivide,
    AddSubstract,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;

        match *self {
            Caret => Power,
            Multiply | Divide => MultiplyDivide,
            Add | Substract => AddSubstract,
            _ => DefaultZero,
        }
    }
}