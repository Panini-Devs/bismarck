use std::iter::{self, Peekable};

#[derive(Clone, Debug, PartialEq)]
pub enum Opc {
    Plus,
    Minus,
    Star,
    Slash,
}

impl Opc {
    fn new(c: char) -> Option<Self> {
        match c {
            '+' => Some(Opc::Plus),
            '-' => Some(Opc::Minus),
            '*' => Some(Opc::Star),
            '/' => Some(Opc::Slash),
            _ => None,
        }
    }

    // The precedence of the operator, higher means that operation has priority
    fn pred(&self) -> i32 {
        match self {
            Opc::Plus => 0,
            Opc::Minus => 0,
            Opc::Star => 1,
            Opc::Slash => 1,
        }
    }

    // How one operator combines two values
    fn apply(&self, l: f64, r: f64) -> Result<Value, &'static str> {
        match self {
            Opc::Plus => l.add(r),
            Opc::Minus => l.sub(r),
            Opc::Star => l.mul(r),
            Opc::Slash => l.div(r),
        }
    }
}
