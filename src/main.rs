mod error;
mod lexer;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), Some(std::io::stdout()));
    while lexer.read('>').unwrap() > 0 {
        let vec: Result<Vec<token::Token>, error::Error> = (&mut lexer).collect();
        println!("{:?}", vec);
    }
    println!();
}
