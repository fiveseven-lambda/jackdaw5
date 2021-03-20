mod pos;
mod score;

fn main() {
    match std::fs::read_to_string("test") {
        Ok(string) => match score::lexer(&string) {
            Ok(tokens) => match score::parse_expression(&mut tokens.iter()) {
                Ok((_, result)) => {
                    println!("{:?}", result.value(&std::collections::HashMap::new()));
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
