mod pos;
mod score;

fn main() {
    match std::fs::read_to_string("test") {
        Ok(string) => match score::lexer(&string) {
            Ok(tokens) => match score::parse(&mut tokens.iter()) {
                Ok(result) => {
                    println!("{:?}", result);
                }
                Err(err) => {
                    println!("{}", err);
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
