mod error;
mod lexer;
mod pos;
mod token;

fn main() {
    for result in lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true) {
        match result {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }
}
