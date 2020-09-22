use nom::IResult;
use nom::character::complete::char;
use crate::calclator::ast::expression::Expression;
use super::constant_parser::constant_parser;

/// parser for Expression
pub fn expression_parser(sent: &str) -> IResult<&str, Expression> {
    constant_parser(sent)
        .map(|(no_used, parsed)| (no_used, Expression::Constant(parsed)))
}

/// parser for Expression with parenthses
pub fn expression_parser_with_paren(sent: &str) -> IResult<&str, Expression> {
    let (no_used, _) = char('(')(sent)?;
    let (no_used, expression) = expression_parser(no_used)?;
    let (no_used, _) = char(')')(no_used)?;

    Ok((no_used, expression))
}
