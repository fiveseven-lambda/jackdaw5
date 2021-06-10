mod error;
mod lexer;
mod pos;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    loop {
        match lexer.next() {
            Ok(Some(token)) => println!("{:?}", token),
            Ok(None) => break,
            Err(err) => break println!("{}", err),
        }
    }
}
