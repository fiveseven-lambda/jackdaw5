mod ast;
mod error;
mod lexer;
mod parser;
mod pos;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    match parser::parse_expression(&mut lexer) {
        Ok(expression) => {
            println!("{:#?}", expression);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
