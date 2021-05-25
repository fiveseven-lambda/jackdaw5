mod ast;
mod error;
mod lexer;
mod pos;
mod token;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);

    /*
    match parser::parse_expression(&mut lexer) {
        Ok(expression) => {
            println!("{:#?}", expression);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    */
}
