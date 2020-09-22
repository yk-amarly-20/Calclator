// parser for struct Constant

use crate::calclator::ast::constant::Constant;
use nom::IResult;
use std::str::FromStr;
use nom::character::complete::digit1;

/// parser for Constant
pub fn constant_parser(sent: &str) -> IResult<&str, Constant> {
    let (no_used, used) = digit1(sent)?;
    let value = FromStr::from_str(used).unwrap();
    Ok((no_used, Constant::new(value)))
}
