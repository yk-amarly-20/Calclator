use nom::IResult;
use nom::combinator::{map, opt};
use nom::branch::alt;
use nom::sequence::tuple;
use nom::character::complete::char;
use crate::calclator::ast::expression::Expression;
use crate::calclator::ast::operation::Operation;
use crate::calclator::ast::operation::BinaryOperation;
use super::factor_parser::factor_parser;

/// factor (* | /) term
pub fn term_parser(sent: &str) -> IResult<&str, Expression> {

    let operation_parser =
        map(
            // convert char to Operation
            alt((char('*'), char('/'))),
            |option|
                match option {
                    '*' => Operation::_Times,
                    '/' => Operation::_Divided,
                    _ => panic!("No Operation!!"),
                },
        );

    // *, / => term
    let binary_operation_parser = tuple((
        factor_parser,
        opt(
            tuple((
                operation_parser,
                term_parser
            ))
        )
    ));

    map(binary_operation_parser, |(head_expression, tail_operation)| {
        if let Option::Some((operation, tail_expression)) = tail_operation {
            Expression::BinaryOperation(
                Box::new(
                    BinaryOperation::new(operation, head_expression, tail_expression)
                )
            )
        } else {
            head_expression
        }
    })(sent)
}
