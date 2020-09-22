// parser
pub mod constant_parser;
pub mod expression_parser;

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
