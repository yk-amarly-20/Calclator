use super::expression::Expression;
use super::constant;

/// operation
#[derive(Debug, PartialEq)]
pub enum Operation {
    _Plus,
    _Minus,
    _Times,
    _Divided,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    operation: Operation,
    left: Expression,
    right: Expression,
}

impl BinaryOperation {
    pub fn new(operation: Operation, left: Expression, right: Expression) -> BinaryOperation {
        BinaryOperation {operation, left, right}
    }

    pub fn eval(&self) -> i32 {
        let left = self.left.eval();
        let right = self.right.eval();

        match self.operation {
            Operation::_Plus => left + right,
            Operation::_Minus => left - right,
            Operation::_Times => left * right,
            Operation::_Divided => left / right,
        }
    }
}

#[test]
fn operation_test() {
    let binary_operation = BinaryOperation::new(
        Operation::_Times,
        Expression::Constant(constant::Constant::new(15)),
        Expression::BinaryOperation(
            Box::new(
                BinaryOperation::new(
                    Operation::_Minus,
                    Expression::Constant(constant::Constant::new(19)),
                    Expression::Constant(constant::Constant::new(7)),
                )
            )
        ),
    );

    let expect = 15 * (19 - 7);
    assert_eq!(
        expect,
        binary_operation.eval()
    );
}
