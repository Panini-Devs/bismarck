use fast_float::parse_partial;

/// Struct describing a mathematical operator that takes two operands
pub struct BinaryOperator<'a> {
    /// The precedence of an determines where it is located in the priority chain.
    ///
    /// Multiplication has higher precedence than addition,
    /// so multiplication happens before addition.
    ///
    /// A limitation is imposed: All unary operators have higher precedence than any binary operator
    /// no matter how higher is the precedence value of the binary operator.
    pub precedence: i32,

    /// The identifier of the operator, not limited to just one character.
    /// A symbol can be defined as an operator, such that `5 and 1` would evaluate to `1`
    /// (boolean addition)
    pub identifier: &'a str,

    /// Operator application method
    ///
    /// # Arguments
    ///
    /// * `left_operand` - The number that appears at the left of the operator
    /// * `right_operand` - The number that appears at the right of the operator
    ///
    /// # Returns
    ///
    /// Result of applying the operator to the two operands.
    /// The return type is a `Result<f64, &'static str>` for the application may error,
    /// for example in cases where division by zero happens.
    pub apply: fn(left_operand: f64, right_operand: f64) -> Result<f64, &'static str>,
}

/// Struct describing a mathematical operator that takes one operand
pub struct UnaryOperator {
    /// Identifier of the operator
    pub identifier: char,

    /// Operator application method
    ///
    /// # Arguments
    ///
    /// * `operand` - Number to apply the operator to
    ///
    /// # Returns
    ///
    /// Result of applying the operator to the operand.
    /// One common error to signal is overflow, for example, caused by `50!`.
    pub apply: fn(operand: f64) -> Result<f64, &'static str>,
}

/// Operator environment for a mathematical expression
pub struct OperatorTable<'a> {
    pub prefix: &'a [UnaryOperator],
    pub infix: &'a [BinaryOperator<'a>],
}

/// Result of evaluating a mathematical expression
///
/// # Grammar
/// As of now, an expression is anything that complies with the following grammar
/// ```
/// expr: number
///     | prefix_operator expr
///     | '(' expr ')'
///     | expr infix_operator expr
///     ;
///
/// number: /* anything parseable by fast_float::parse_partial */
///       ;
/// ```
type Value = f64;

fn parse_number(data: &str) -> Option<(&str, Value)> {
    parse_partial(data)
        .map(|(num, readcount)| (&data[readcount..], num))
        .ok()
}

fn parse_primary<'a>(data: &'a str, env: &OperatorTable) -> Option<(&'a str, Value)> {
    data.chars()
        .skip_while(|c| c.is_whitespace())
        .next()
        .and_then(|cur_char| {
            env.prefix
                .iter()
                .find(|op| op.identifier == cur_char)
                .and_then(|op| {
                    parse_primary(&data[1..], env)
                        .and_then(|(data, value)| (op.apply)(value).map(|v| (data, v)).ok())
                })
                .or_else(|| parse_number(data))
        })
}

fn parse_op<'a>(
    data: &'a str,
    env: &'a OperatorTable,
) -> Option<(&'a str, &'a BinaryOperator<'a>)> {
    let data = &data[data.find(|c: char| !c.is_whitespace())?..];
    let op = env.infix.iter().max_by_key(|op| {
        op.identifier
            .chars()
            .zip(data.chars())
            .take_while(|(a, b)| a == b)
            .count()
    })?;
    Some((data, op))
}

fn parse_expression<'a>(
    mut data: &'a str,
    mut lhs: Value,
    env: &'a OperatorTable,
    pred: i32,
) -> Option<(&'a str, Value)> {
    loop {
        let op;
        (data, op) = match parse_op(data, env) {
            Some((_, op)) if op.precedence < pred => break,
            Some(t) => t,
            None => break,
        };

        let mut rhs;
        (data, rhs) = parse_primary(data, env)?;
        loop {
            let op2;
            (data, op2) = match parse_op(data, env) {
                Some((_, op2)) if op2.precedence <= op.precedence => break,
                Some(t) => t,
                None => break,
            };
            (data, rhs) = parse_expression(data, rhs, env, op2.precedence)?;
        }
        lhs = (op.apply)(lhs, rhs).ok()?;
    }
    Some((data, lhs))
}

pub fn eval<'a>(data: &'a str, env: &'a OperatorTable) -> Option<Value> {
    let (data, p) = parse_primary(data, env)?;
    let (data, e) = parse_expression(data, p, env, 0)?;
    match data.chars().find(|c| !c.is_whitespace()) {
	None => Some(e),
	Some(_) => None,
    }
}
