mod ast;
mod error;
mod lexer;
mod pos;
mod token;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    loop {
        match lexer.next() {
            Ok(Some(token)) => {
                println!("{:?}", token);
            }
            Ok(None) => break,
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
    }
}
