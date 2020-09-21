extern crate calclator;
use calclator::calclator::ast;

fn main() {
    let a = ast::constant::Constant::new(22);

    println!("Constant={}", a.eval());
}
