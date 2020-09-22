// parser
use nom::character::complete::digit1;
use nom::IResult;

#[test]
fn digit_test() {
    let phrase = "35thu";
    let result: IResult<&str, &str> = digit1(&phrase);
    let (no_used, used) = result.unwrap();
    assert_eq!("35", used);
    assert_eq!("thu", no_used);
}
