mod error;
mod lexer;
mod pos;
mod token;

fn main() {
    for token in lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true) {
        println!("{:?}", token);
    }
}
