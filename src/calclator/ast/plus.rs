use expression::Expression;

/// plus operation
pub struct Plus {
    left: Expression,
    right: Expression,
}

/// implementation of plus
impl Plus {
    /// generate plus operation
    pub fn new(left: Expression, right: Expression) -> Plus {
        Plus {left, right}
    }

    /// evaluate
    pub fn eval(&self) -> i32 {
        self.left.eval() + self.right.eval()
    }
}

#[test]
fn plus_test() {
    let plus = Plus::new(
        Expression::Constant(Constant::new(10)),
        Expression::Plus(
            Box::new(
                Plus::new(
                    Expression::Constant(Constant::new(3)),
                    Expression::Constant(Constant::new(8))
                )
            )
        ),
    );

    let expect = 10 + (3 + 8);

    assert_eq!(plus.eval(), expect);
}
