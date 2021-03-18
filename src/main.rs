mod score;

fn main() {
    match score::read(&std::path::PathBuf::from("test")) {
        Ok(score) => match score::lexer(&score) {
            Ok(tokens) => {
                match score::parse_expression(&mut tokens.iter()) {
                    Ok(parsed) => {
                        println!("{:#?}", parsed);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
