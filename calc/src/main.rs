use std::io;

mod parsemath;
use parsemath::ast;
use parsemath::parser::{Parser, ParseError};

fn main() {
    println!("Hello! Welcome to Arithmetic expression evaluator.");
    println!("You can calculate value for expression such as 2 * (3 + 4) / 5");
    println!("Allowed number: positive, negative and decimals.");
    println!("Allowed operators: +, -, *, /, ^");
    println!("Enter your arithmetic expression below:");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(result) => println!("The computed number is {}\n", result),
                    Err(_) => println!("Error in evaluating expression.Please enter valid expression\n"),
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}