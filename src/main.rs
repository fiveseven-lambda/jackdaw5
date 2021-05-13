mod error;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    for c in lexer {
        println!("{}", c.unwrap());
    }
}
