mod error;
mod token;
mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new();
    lexer.add("abc".to_string());
}
