use constant::Constant;
use plus::Plus;

/// Expression
pub enum Expression {
    Constant(Constant),
    Plus(Box<Plus>),
}

impl Expression {
    /// evaluate expression
    pub fn eval(&self) -> i32 {
        match self {
            Expression::Constant(e) => e.eval(),
            Expression::Plus(e) => e.eval(),
        }
    }
}
