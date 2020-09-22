use super::constant::Constant;
// use super::plus::Plus;
use super::operation::BinaryOperation;

/// Expression
#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(Constant),
    BinaryOperation(Box<BinaryOperation>),
}

impl Expression {
    /// evaluate expression
    pub fn eval(&self) -> i32 {
        match self {
            Expression::Constant(e) => e.eval(),
            Expression::BinaryOperation(e) => e.eval(),
        }
    }
}
