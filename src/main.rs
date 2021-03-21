mod pos;
mod score;
use score::Score;

fn main() {
    match std::fs::read_to_string("test") {
        Ok(string) => match Score::from_source(&string) {
            Some(score) => {
                println!("{:?}", score);
            }
            None => {}
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
