mod error;
mod lexer;
mod pos;
mod token;
mod ast;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    loop {
        match parser::parse_expression(&mut lexer) {
            Ok(Some(expression)) => println!("{:#?}", expression),
            Ok(None) => break,
            Err(err) => break println!("{}", err),
        }
    }
}
