pub mod ast;
pub mod parser;

use nom::Err;
use nom::error::ErrorKind;
use parser::expression_parser::expression_parser;

/// evaluate parsed expression
pub fn expression_evaluate(sent: &str) -> Result<i32, Err<(&str, ErrorKind)>> {
    expression_parser(sent)
        .map(|(_, expression)| expression.eval())
}

#[test]
fn test_expression_evaluate() {
    assert_eq!(
        expression_evaluate("1+2+3+4+5").unwrap(),
        1 + 2 + 3 + 4 + 5
    );
}
