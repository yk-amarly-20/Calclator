/// constant
pub struct Constant(i32);

impl Constant {
    /// generate Constant
    pub fn new(value: i32) -> Constant {
        Constant(value)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

#[test]
fn constant_test() {
    let expect = 25;
    let constant = Constant::new(25);
    assert_eq!(
        expect,
        constant.get()
    );
}
