
/// Trait describing a mathematical operator that takes two operands
trait BinaryOperator {
    /// The precedence of an determines where it is located in the priority chain.
    ///
    /// Multiplication has higher precedence than addition,
    /// so multiplication happens before addition.
    ///
    /// A limitation is imposed: All unary operators have higher precedence than any binary operator
    /// no matter how higher is the precedence value of the binary operator.
    const PRECEDENCE: i32;

    /// The identifier of the operator, not limited to just one character.
    /// A symbol can be defined as an operator, such that `5 and 1` would evaluate to `1`
    /// (boolean addition)
    const IDENTIFIER: &'static str;

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
    fn apply(&self, left_operand: f64, right_operand: f64) -> Result<f64, &'static str>;
}

/// Trait describing a mathematical operator that takes one operand
trait UnaryOperator {
    /// Identifier of the operator 
    const IDENTIFIER: char;

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
    fn apply(&self, operand: f64) -> Resutl<f64, &'static str>;
}
