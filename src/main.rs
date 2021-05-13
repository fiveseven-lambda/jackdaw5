mod ast;
mod token;

use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s);
    println!("{:#?}", token::token(&s));
}
