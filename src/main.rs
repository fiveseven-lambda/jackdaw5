mod ast;
mod error;
mod lexer;
mod pos;
mod token;
mod parser;
mod value;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    loop {
        match parser::parse_expression(&mut lexer) {
            Ok(Some(expression)) => match expression.evaluate() {
                Ok(value) => println!("{:?}", value),
                Err(err) => println!("{}", err),
            },
            Ok(None) => break,
            Err(err) => break eprintln!("{}", err),
        }
    }
}
