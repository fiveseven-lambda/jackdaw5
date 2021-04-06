mod lexer;
mod token;

fn main() {
    let stdin = std::io::stdin();
    loop {
        let mut s = String::new();
        match stdin.read_line(&mut s) {
            Ok(0) => break,
            Ok(_) => {
                for token_pos in lexer::Lexer::new(&s) {
                    println!("{:?}", token_pos);
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
    }
}
