// factor parser
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;
use crate::calclator::ast::expression::Expression;
use super::constant_parser::constant_parser;
use super::expression_parser::expression_parser_with_paren;

/// parser for Factor ( = (Constant | Expression) )
pub fn factor_parser(sent: &str) -> IResult<&str, Expression> {
    alt((
        map(
            constant_parser,
            |constant| Expression::Constant(constant)
        ),
        expression_parser_with_paren
    ))(sent)
}
