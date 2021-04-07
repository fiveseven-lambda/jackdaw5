mod error;
mod parse;
mod token;

fn main() {
    let mut input = std::io::BufReader::new(std::io::stdin());
    let mut output = Some(std::io::stdout());
    loop {
        parse::parse(&mut input, &mut output);
    }
}
