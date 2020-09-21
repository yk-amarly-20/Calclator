extern crate calclator;
use calclator::calclator::ast;

fn main() {
    let a = ast::Constant::new(33);

    println!("Constant={}", a.get());
}
