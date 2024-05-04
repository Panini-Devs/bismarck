use std::iter::{self, Peekable};

/*
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    required_permissions = "SEND_MESSAGES",
    aliases("calc", "eval"),
    user_cooldown = 2
)]
pub async fn math(
    context: Context<'_>,
    #[description = "Expression to evaluate"] expr: String,
) -> Result<(), Error> {
    let toks = tokenize(&expr)?;
    let ops = Parser::new(toks).parse()?;
    match execute(ops)? {
        Ret::I32(n) => todo!("Output i32"),
        Ret::U32(n) => todo!("Output u32"),
        Ret::F32(n) => todo!("Output f32"),
    }
    Ok(())
}
*/

mod eval {
    use super::*;

    #[allow(dead_code)]
    fn p(s: &str) -> Value {
        Parser::new(tokenize(s).unwrap().into_iter())
            .parse()
            .unwrap()
    }

    #[allow(unused_imports)]
    mod test_u32 {
        use super::*;

        #[test]
        fn add() {
            assert_eq!(p("1_u + 2_u"), Value::U32(3));
            // assert_eq!(p(&format!("{}_u + 1_u", u32::MAX)), Value::U32(0)); // will panic
        }

        #[test]
        fn sub() {
            assert_eq!(p("2_u - 1_u"), Value::U32(1));
            // assert_eq!(p("0_u - 1_u"), Value::U32(u32::MAX)); // will panic
        }

        #[test]
        fn mul() {
            assert_eq!(p("2_u * 3_u"), Value::U32(6));
        }

        #[test]
        fn div() {
            assert_eq!(p("6_u / 2_u"), Value::U32(3));
        }
    }

    #[allow(unused_imports)]
    mod test_i32 {
        use super::*;

        #[test]
        fn add() {
            assert_eq!(p("1 + 2"), Value::I32(3));
            assert_eq!(p(&format!("{}", i32::MAX)), Value::I32(i32::MAX));
        }

        #[test]
        fn sub() {
            assert_eq!(p("1 - 2"), Value::I32(-1));
            // assert_eq!(p(&format!("{} - 2", i32::MIN + 1)), Value::I32(i32::MAX)); // will panic
        }

        #[test]
        fn mul() {
            assert_eq!(p("2 * 3"), Value::I32(6));
        }

        #[test]
        fn div() {
            assert_eq!(p("6 / 3"), Value::I32(2));
        }
    }

    #[allow(unused_imports)]
    mod test_f32 {
        use super::*;

        #[test]
        fn add() {
            assert_eq!(p("1.0 + 2.4"), Value::F32(3.4));
        }

        #[test]
        fn sub() {
            assert_eq!(p("1. - 2.0"), Value::F32(-1.));
        }

        #[test]
        fn mul() {
            assert_eq!(p("2.0 * 0.5"), Value::F32(1.));
        }

        #[test]
        fn div() {
            assert_eq!(p("6.0 / 0.5"), Value::F32(12.));
        }
    }

    #[allow(unused_imports)]
    mod test_pred {
        use super::*;

        #[test]
        fn chain() {
            assert_eq!(p("1 + 2 + 3"), Value::I32(6));
        }

        #[test]
        fn mulsum() {
            assert_eq!(p("2 + 3 * 4"), Value::I32(2 + 3 * 4));
            assert_eq!(p("3 * 4 + 2"), Value::I32(3 * 4 + 2));
        }

        #[test]
        fn div() {
            assert_eq!(p("2 * 10 / 3"), Value::I32(2 * 10 / 3));
            assert_eq!(p("4 / 2 / 2"), Value::I32(4 / 2 / 2));
        }

        #[test]
        fn paren() {
            assert_eq!(p("(2 + 3) * 4"), Value::I32((2 + 3) * 4));
            assert_eq!(p("4 * (2 + 3)"), Value::I32(4 * (2 + 3)));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    F32(f32),
    U32(u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    I32,
    F32,
    U32,
}

// TODO: Make it checked so it doesn't panic
macro_rules! mkbinop {
    ($name:ident, $op:tt) => {
	fn $name(self, other: Value) -> Result<Value, &'static str> {
	    use Value as V;
            match (self, other) {
		(V::I32(l), V::I32(r)) => Ok(V::I32(l $op r)),
		(V::U32(l), V::U32(r)) => Ok(V::U32(l $op r)),
		(V::F32(l), V::F32(r)) => Ok(V::F32(l $op r)),
		_ => Err("Type mismatch"),
            }
	}
    }
}

impl Value {
    fn cast(self, typ: Type) -> Result<Value, &'static str> {
        use Type as T;
        use Value as V;
        Ok(match (self.clone(), typ) {
            (V::I32(n), T::F32) => V::F32(n as f32),
            (V::I32(n), T::U32) => {
                if n < 0 {
                    Err("Cannot convert negative number to unsigned")?
                } else {
                    V::U32(n as u32)
                }
            }
            (V::I32(_), T::I32) => self,

            (V::U32(n), T::F32) => V::F32(n as f32),
            (V::U32(_), T::U32) => self,
            (V::U32(n), T::I32) => {
                if n > i32::MAX as u32 {
                    Err("Number too big to fit in signed integer")?
                } else {
                    V::I32(n as i32)
                }
            }

            (V::F32(_), T::F32) => self,
            (V::F32(n), T::U32) => V::U32(n as u32),
            (V::F32(n), T::I32) => V::I32(n as i32),
        })
    }

    fn neg(self) -> Result<Value, &'static str> {
        use Value as V;
        match self {
            V::I32(n) => Ok(V::I32(-n)),
            V::U32(_) => Err("Cannot negate unsigned value"),
            V::F32(n) => Ok(V::F32(-n)),
        }
    }

    mkbinop!(mul, *);
    mkbinop!(add, +);
    mkbinop!(sub, -);
    mkbinop!(div, /);
}

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
    fn apply(&self, l: Value, r: Value) -> Result<Value, &'static str> {
        match self {
            Opc::Plus => l.add(r),
            Opc::Minus => l.sub(r),
            Opc::Star => l.mul(r),
            Opc::Slash => l.div(r),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Eof,
    LParen,
    RParen,
    Operator(Opc),
    Symbol(String),
    I32(i32),
    F32(f32),
    Cast(Type),
}

// A simple lexer
pub fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut it = input.chars().peekable();
    let mut v: Vec<Token> = Vec::new();

    while let Some(c) = it.next() {
        match c {
            _ if c.is_whitespace() => (),
            _ if Opc::new(c).is_some() => v.push(Token::Operator(Opc::new(c).unwrap())),
            '(' => v.push(Token::LParen),
            ')' => v.push(Token::RParen),
            '_' => match it.next() {
                Some('i') => v.push(Token::Cast(Type::I32)),
                Some('f') => v.push(Token::Cast(Type::F32)),
                Some('u') => v.push(Token::Cast(Type::U32)),
                _ => return Err("Invalid use of '_' (casting operator)"),
            },
            _ if c.is_ascii_digit() => {
                let n: String = iter::once(c)
                    .chain(iter::from_fn(|| {
                        it.by_ref().next_if(|c| c.is_ascii_digit())
                    }))
                    .collect();
                if it.next_if_eq(&'.').is_some() {
                    let f: String = n
                        .chars()
                        .chain(iter::once('.'))
                        .chain(iter::from_fn(|| {
                            it.by_ref().next_if(|c| c.is_ascii_digit())
                        }))
                        .collect();
                    match f.parse::<f32>() {
                        Err(_) => return Err("Invalid float format"),
                        Ok(f) => v.push(Token::F32(f)),
                    }
                } else {
                    match n.parse::<i32>() {
                        Err(_) => return Err("Number too big"),
                        Ok(n) => v.push(Token::I32(n)),
                    }
                }
            }
            _ if c.is_alphabetic() => v.push(Token::Symbol(
                iter::from_fn(|| it.by_ref().next_if(|c| c.is_alphanumeric())).collect(),
            )),
            _ => return Err("Invalid character"),
        }
    }
    Ok(v)
}

// A recursive descent parser, see https://en.wikipedia.org/wiki/Recursive_descent_parser
pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(toks: I) -> Self {
        Self {
            tokens: toks.peekable(),
        }
    }

    fn next(&mut self) -> Token {
        self.tokens.next().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.peek().cloned().unwrap_or(Token::Eof)
    }

    pub fn parse(mut self) -> Result<Value, &'static str> {
        let p = self.primary()?;
        let e = self.expression(p, 0)?;
        match self.next() {
            Token::Eof => Ok(e),
            _ => Err("Expected end of input"),
        }
    }

    // A pratt parser, see https://en.wikipedia.org/wiki/Operator-precedence_parser
    fn expression(&mut self, mut lhs: Value, pred: i32) -> Result<Value, &'static str> {
        loop {
            let op = match self.peek() {
                Token::Operator(op) if op.pred() >= pred => op,
                _ => break,
            };
            self.next();

            let mut rhs = self.primary()?;
            loop {
                let op2 = match self.peek() {
                    Token::Operator(op2) if op2.pred() > op.pred() => op2,
                    _ => break,
                };
                rhs = self.expression(rhs, op2.pred())?;
                self.next();
            }
            lhs = op.apply(lhs, rhs)?;
        }
        Ok(lhs)
    }

    fn primary(&mut self) -> Result<Value, &'static str> {
        let r = match self.next() {
            Token::LParen => {
                let p = self.primary()?;
                let r = self.expression(p, 0)?;
                match self.next() {
                    Token::RParen => r,
                    _ => Err("Expected ')'")?,
                }
            }
            Token::Operator(Opc::Minus) => self.primary()?.neg()?,
            Token::Operator(Opc::Plus) => self.primary()?,
            Token::F32(n) => Value::F32(n),
            Token::I32(n) => Value::I32(n),
            _ => Err("Expected a value")?,
        };
        Ok(match self.peek() {
            Token::Cast(t) => {
                self.next();
                r.cast(t)?
            }
            _ => r,
        })
    }
}
