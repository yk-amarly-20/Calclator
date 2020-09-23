// parser
pub mod constant_parser;
pub mod expression_parser;
pub mod factor_parser;
pub mod term_parser;

#[test]
fn digit_test() {
    use nom::character::complete::digit1;
    use nom::IResult;

    let phrase = "35thu";
    let result: IResult<&str, &str> = digit1(&phrase);
    let (no_used, used) = result.unwrap();
    assert_eq!("35", used);
    assert_eq!("thu", no_used);
}

#[test]
fn test_constant_parser() {
    use constant_parser::constant_parser;
    use crate::calclator::ast::constant::Constant;

    let sent = "80";
    let (_, parsed) = constant_parser(sent).unwrap();
    let expect = Constant::new(80);

    assert_eq!(parsed, expect);
}

#[test]
fn test_expression_parser_with_paren() {
    use expression_parser::expression_parser_with_paren;
    use crate::calclator::ast::expression::Expression;
    use crate::calclator::ast::constant::Constant;

    let sent = "(80)";
    let (_, parsed) = expression_parser_with_paren(sent).unwrap();
    let expect = Expression::Constant(Constant::new(80));
    assert_eq!(parsed, expect);
}

#[test]
fn test_factor_parser() {
    use factor_parser::factor_parser;
    use crate::calclator::ast::expression::Expression;
    use crate::calclator::ast::constant::Constant;

    let (_, parsed) = factor_parser("7").unwrap();
    let expect = Expression::Constant(Constant::new(7));
    assert_eq!(parsed, expect);

    let (_, parsed) = factor_parser("(8)").unwrap();
    let expect = Expression::Constant(Constant::new(8));
    assert_eq!(parsed, expect);
}

#[test]
fn test_term_parser() {
    use term_parser::term_parser;
    use crate::calclator::ast::expression::Expression;
    use crate::calclator::ast::constant::Constant;
    use crate::calclator::ast::operation::{BinaryOperation, Operation};

    let (_, parsed) = term_parser("4/2*3").unwrap();
    let tmp = Expression::BinaryOperation(
        Box::new(BinaryOperation::new(
            Operation::_Times,
            Expression::Constant(Constant::new(2)),
            Expression::Constant(Constant::new(3))
            )
        )
    );

    let expected = Expression::BinaryOperation(
        Box::new(BinaryOperation::new(
            Operation::_Divided,
            Expression::Constant(Constant::new(4)),
            tmp
        ))
    );

    assert_eq!(parsed, expected);
}
