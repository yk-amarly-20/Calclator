use nom::IResult;
use crate::calclator::ast::expression::Expression;
use super::constant_parser::constant_parser;

/// parser for Expression
pub fn expression_parser(sent: &str) -> IResult<&str, Expression> {
    constant_parser(sent)
        .map(|(no_used, parsed)| (no_used, Expression::Constant(parsed)))
}
