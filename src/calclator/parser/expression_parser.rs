use nom::IResult;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::branch::alt;
use nom::sequence::tuple;
use crate::calclator::ast::expression::Expression;
use crate::calclator::ast::operation::{BinaryOperation, Operation};
// use super::constant_parser::constant_parser;
use super::term_parser::term_parser;
// use super::factor_parser::factor_parser;

/// parser for Expression
pub fn expression_parser(sent: &str) -> IResult<&str, Expression> {
    let operational_parser = map(
        alt((char('+'), char('-'))),
        |option|
            match option {
                '+' => Operation::_Plus,
                '-' => Operation::_Minus,
                _ => panic!("No Operation!!"),
            }
    );

    let binary_operation_parser = tuple((
        term_parser,
        opt(
            tuple((
                operational_parser,
                expression_parser
            ))
        )
    ));

    map(binary_operation_parser, |(head_expression, tail_operation)| {
        if let Option::Some((operation, tail_expression)) = tail_operation {
            Expression::BinaryOperation(
                Box::new(
                    BinaryOperation::new(
                        operation,
                        head_expression,
                        tail_expression
                    )
                )
            )
        } else {
            head_expression
        }
    })(sent)
}

/// parser for Expression with parenthses
pub fn expression_parser_with_paren(sent: &str) -> IResult<&str, Expression> {
    let (no_used, _) = char('(')(sent)?;
    let (no_used, expression) = expression_parser(no_used)?;
    let (no_used, _) = char(')')(no_used)?;

    Ok((no_used, expression))
}
