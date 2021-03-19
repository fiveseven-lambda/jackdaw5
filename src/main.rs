mod pos;
mod score;

fn main() {
    match std::fs::read_to_string("test") {
        Ok(string) => {
            score::lexer(&string);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
