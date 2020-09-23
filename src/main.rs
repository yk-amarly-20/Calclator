extern crate calclator;
use calclator::calclator::expression_evaluate;

fn main() {
    loop {
        let mut expression = String::new();
        println!("Insert Expression");
        std::io::stdin().read_line(&mut expression).ok();
        match expression_evaluate(&expression) {
            Ok(value) => println!("Answer: {}", value),
            _ => println!("Bad Insert!"),
        }
    }
}
