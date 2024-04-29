use std::{iter, mem::transmute};

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

    mod test_u32 {
        use super::*;

        #[allow(dead_code)]
        fn tst(s: &str, e: u32) {
            let toks = tokenize(s).unwrap();
            let ops = Parser::new(toks).parse().unwrap();
            let r = execute(ops).unwrap();
            assert_eq!(r, Ret::U32(e));
        }

        #[test]
        fn add() {
            tst("1_u + 2_u", 3);
            tst(&format!("{}_u + 1_u", u32::MAX), 0);
        }

        #[test]
        fn sub() {
            tst("2_u - 1_u", 1);
            tst("0_u - 1_u", u32::MAX);
        }

        #[test]
        fn mul() {
            tst("2_u * 3_u", 6);
        }

        #[test]
        fn div() {
            tst("6_u / 2_u", 3);
        }
    }

    mod test_i32 {
        use super::*;

        #[allow(dead_code)]
        fn tst(s: &str, e: i32) {
            let toks = tokenize(s).unwrap();
            let ops = Parser::new(toks).parse().unwrap();
            let r = execute(ops).unwrap();
            assert_eq!(r, Ret::I32(e));
        }

        #[test]
        fn add() {
            tst("1 + 2", 3);
            tst(&format!("{}", i32::MAX), i32::MAX);
        }

        #[test]
        fn sub() {
            tst("1 - 2", -1);
            tst(&format!("{} - 1", i32::MIN), i32::MAX);
        }

        #[test]
        fn mul() {
            tst("2 * 3", 6);
        }

        #[test]
        fn div() {
            tst("6 / 3", 2);
        }
    }

    mod test_f32 {
        use super::*;

        #[allow(dead_code)]
        fn tst(s: &str, e: f32) {
            let toks = tokenize(s).unwrap();
            let ops = Parser::new(toks).parse().unwrap();
            let r = execute(ops).unwrap();
            assert_eq!(r, Ret::F32(e));
        }

        #[test]
        fn add() {
            tst("1.0 + 2.4", 3.4);
        }

        #[test]
        fn sub() {
            tst("1. - 2.0", -1.);
        }

        #[test]
        fn mul() {
            tst("2.0 * 0.5", 1.);
        }

        #[test]
        fn div() {
            tst("6.0 / 0.5", 12.);
        }
    }

    mod test_pred {
        use super::*;
        #[allow(dead_code)]
        fn tst(s: &str, e: i32) {
            let toks = tokenize(s).unwrap();
            let ops = Parser::new(toks).parse().unwrap();
            let r = execute(ops).unwrap();
            assert_eq!(r, Ret::I32(e));
        }

        #[test]
        fn chain() {
            tst("1 + 2 + 3", 6);
        }

        #[test]
        fn mulsum() {
            tst("2 + 3 * 4", 2 + 3 * 4);
            tst("3 * 4 + 2", 3 * 4 + 2);
        }

        #[test]
        fn div() {
            tst("2 * 10 / 3", 2 * 10 / 3);
            tst("4 / 2 / 2", 4 / 2 / 2);
        }

        #[test]
        fn paren() {
            tst("(2 + 3) * 4", (2 + 3) * 4);
            tst("4 * (2 + 3)", 4 * (2 + 3));
        }
    }
}

#[derive(Debug, Clone)]
#[repr(u32)]
enum Operation {
    Goto,
    Test,
    Nop,

    Const32,

    ExitF32,
    ExitI32,
    ExitU32,

    F32Neg,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Lt,
    F32Eq,
    F32CastI32S,
    F32CastI32U,

    I32Neg,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32Lt,
    I32Eq,
    I32CastF32S,
    I32CastF32U,
}

struct Stack {
    data: Vec<u32>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn pop(&mut self) -> u32 {
        unsafe { self.data.pop().unwrap_unchecked() }
    }

    fn pops(&mut self) -> i32 {
        unsafe { transmute::<u32, i32>(self.pop()) }
    }

    fn popf(&mut self) -> f32 {
        unsafe { transmute::<u32, f32>(self.pop()) }
    }

    fn push(&mut self, v: u32) {
        self.data.push(v);
    }

    fn pushb(&mut self, v: bool) {
        self.push(v as u32);
    }

    fn pushf(&mut self, v: f32) {
        self.push(unsafe { transmute::<f32, u32>(v) });
    }

    fn pushs(&mut self, v: i32) {
        self.push(unsafe { transmute::<i32, u32>(v) });
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Ret {
    I32(i32),
    F32(f32),
    U32(u32),
}

struct ByteCode(Vec<u32>);

impl std::fmt::Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut asnum = false;
        for code in self.0.iter() {
            if asnum {
                write!(f, "{:x}\n", code)?;
                asnum = false;
            } else {
                write!(f, "{:?}", unsafe { transmute::<u32, Operation>(*code) })?;
                if *code == Operation::Const32 as u32 {
                    asnum = true;
                    write!(f, " ")?;
                } else {
                    write!(f, "\n")?;
                }
            }
        }
        Ok(())
    }
}

fn execute(operations: ByteCode) -> Result<Ret, &'static str> {
    let mut i = 0usize;
    let mut s = Stack::new();

    loop {
        use Operation as Op;
        let e = operations.0[i];
        match unsafe { transmute::<u32, Operation>(e) } {
            Op::Goto => {
                i = s.pop() as usize;
                continue;
            }
            Op::Test => {
                if s.pop() == 0 {
                    i = i + 1;
                }
            }
            Op::Nop => (),
            Op::Const32 => {
                i = i + 1;
                s.push(operations.0[i]);
            }
            Op::I32CastF32S => {
                let a = s.pops();
                s.pushf(a as f32);
            }
            Op::I32CastF32U => {
                let a = s.pop();
                s.pushf(a as f32);
            }
            Op::F32CastI32S => {
                let a = s.popf();
                s.pushs(a as i32);
            }
            Op::F32CastI32U => {
                let a = s.popf();
                s.push(a as u32);
            }
            Op::ExitI32 => {
                return Ok(Ret::I32(s.pops()));
            }
            Op::ExitF32 => {
                return Ok(Ret::F32(s.popf()));
            }
            Op::ExitU32 => {
                return Ok(Ret::U32(s.pop()));
            }
            Op::I32Neg => {
                let a = s.pop();
                s.push((0u32).wrapping_sub(a));
            }
            Op::F32Neg => {
                let a = s.popf();
                s.pushf(-a);
            }
            binop => {
                let r = s.pop();
                let l = s.pop();
                fn asf32(arg: u32) -> f32 {
                    unsafe { transmute::<u32, f32>(arg) }
                }

                fn asi32(arg: u32) -> i32 {
                    unsafe { transmute::<u32, i32>(arg) }
                }

                match binop {
                    Op::I32Add => s.push(l.wrapping_add(r)),
                    Op::I32Sub => s.push(l.wrapping_sub(r)),
                    Op::I32Mul => s.push(l.wrapping_mul(r)),
                    Op::I32DivS => s.pushs(asi32(l) / asi32(r)),
                    Op::I32DivU => {
                        if r == 0 {
                            return Err("Division by zero");
                        } else {
                            s.push(l / r);
                        }
                    }
                    Op::I32Lt => {
                        if r == 0 {
                            return Err("Division by zero");
                        } else {
                            s.pushb(l < r);
                        }
                    }
                    Op::I32Eq => s.pushb(l == r),

                    Op::F32Add => s.pushf(asf32(l) + asf32(r)),
                    Op::F32Sub => s.pushf(asf32(l) - asf32(r)),
                    Op::F32Mul => s.pushf(asf32(l) * asf32(r)),
                    Op::F32Div => s.pushf(asf32(l) / asf32(r)),
                    Op::F32Eq => s.pushb(asf32(l) == asf32(r)),
                    Op::F32Lt => s.pushb(asf32(l) < asf32(r)),

                    Op::Goto
                    | Op::Test
                    | Op::Const32
                    | Op::I32CastF32S
                    | Op::I32CastF32U
                    | Op::F32CastI32S
                    | Op::F32CastI32U
                    | Op::ExitI32
                    | Op::ExitF32
                    | Op::I32Neg
                    | Op::F32Neg
                    | Op::ExitU32
                    | Op::Nop => {
                        unreachable!()
                    }
                }
            }
        }
        i = i + 1;
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Debug)]
enum Type {
    I32,
    F32,
    U32,
}

impl Type {
    fn cast(&self, typ: Type) -> Operation {
        use Operation::*;
        use Type as T;
        match (self, typ) {
            (T::I32, T::F32) => I32CastF32S,
            (T::U32, T::F32) => I32CastF32U,

            (T::F32, T::I32) => F32CastI32S,
            (T::F32, T::U32) => F32CastI32U,
            _ => Nop,
        }
    }

    fn neg(&self) -> Operation {
        match self {
            Type::I32 | Type::U32 => Operation::I32Neg,
            Type::F32 => Operation::F32Neg,
        }
    }

    fn mul(&self) -> Operation {
        match self {
            Type::I32 | Type::U32 => Operation::I32Mul,
            Type::F32 => Operation::F32Mul,
        }
    }

    fn add(&self) -> Operation {
        match self {
            Type::I32 | Type::U32 => Operation::I32Add,
            Type::F32 => Operation::F32Add,
        }
    }

    fn sub(&self) -> Operation {
        match self {
            Type::I32 | Type::U32 => Operation::I32Sub,
            Type::F32 => Operation::F32Sub,
        }
    }

    fn div(&self) -> Operation {
        match self {
            Type::I32 => Operation::I32DivS,
            Type::U32 => Operation::I32DivU,
            Type::F32 => Operation::F32Div,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Opc {
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

    fn pred(&self) -> i32 {
        match self {
            Opc::Plus => 0,
            Opc::Minus => 0,
            Opc::Star => 1,
            Opc::Slash => 1,
        }
    }

    fn code(&self, typ: Type) -> Operation {
        match self {
            Opc::Plus => typ.add(),
            Opc::Minus => typ.sub(),
            Opc::Star => typ.mul(),
            Opc::Slash => typ.div(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Eof,
    LParen,
    RParen,
    Operator(Opc),
    Symbol(String),
    I32(u32),
    F32(f32),
    Cast(Type),
}

fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
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
                    match n.parse::<u32>() {
                        Err(_) => return Err("How did we get here? (Invalid int format)"),
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

struct Parser {
    output: ByteCode,
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(toks: Vec<Token>) -> Self {
        Self {
            output: ByteCode(Vec::new()),
            tokens: toks,
            index: 0,
        }
    }

    fn add_op(&mut self, code: Operation) {
        self.output.0.push(code as u32);
    }

    fn add_32(&mut self, v: u32) {
        self.output.0.push(v);
    }

    fn peek(&mut self) -> Token {
        self.tokens
            .get(self.index)
            .map(|t| t.clone())
            .unwrap_or(Token::Eof)
    }

    fn eat(&mut self) {
        self.index = self.index + 1
    }

    fn next(&mut self) -> Token {
        let t = self.peek();
        self.eat();
        t
    }

    fn parse(mut self) -> Result<ByteCode, &'static str> {
        let p = self.primary()?;
        let e = self.expression(p, 0)?;
        match self.next() {
            Token::Eof => {
                match e {
                    Type::I32 => self.add_op(Operation::ExitI32),
                    Type::U32 => self.add_op(Operation::ExitU32),
                    Type::F32 => self.add_op(Operation::ExitF32),
                }
                Ok(self.output)
            }
            _ => Err("Expected end of input"),
        }
    }

    fn expression(&mut self, mut lhs: Type, pred: i32) -> Result<Type, &'static str> {
        loop {
            let op = match self.peek() {
                Token::Operator(op) if op.pred() >= pred => op,
                _ => break,
            };
            self.eat();

            let mut rhs = self.primary()?;
            loop {
                let op2 = match self.peek() {
                    Token::Operator(op2) if op2.pred() > op.pred() => op2,
                    _ => break,
                };
                rhs = self.expression(rhs, op2.pred())?;
                self.next();
            }
            if rhs != lhs {
                return Err("Type mismatch");
            }
            self.add_op(op.code(lhs));
            lhs = rhs;
        }
        Ok(lhs)
    }

    fn primary(&mut self) -> Result<Type, &'static str> {
        let r = match self.next() {
            Token::LParen => {
                let p = self.primary()?;
                let r = self.expression(p, 0)?;
                match self.next() {
                    Token::RParen => r,
                    _ => Err("Expected ')'")?,
                }
            }
            Token::Operator(Opc::Minus) => {
                let p = self.primary()?;
                self.add_op(p.neg());
                p
            }
            Token::Operator(Opc::Plus) => {
                let p = self.primary()?;
                p
            }
            Token::F32(n) => {
                self.add_op(Operation::Const32);
                self.add_32(unsafe { transmute::<f32, u32>(n) });
                Type::F32
            }
            Token::I32(n) => {
                self.add_op(Operation::Const32);
                self.add_32(n);
                Type::I32
            }
            _ => Err("Expected a value")?,
        };
        Ok(match self.peek() {
            Token::Cast(t) => {
                self.eat();
                match r.cast(t.clone()) {
                    Operation::Nop => (),
                    t => self.add_op(t),
                }
                t
            }
            _ => r,
        })
    }
}
