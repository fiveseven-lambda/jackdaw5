mod compiler;
mod lexer;
mod parser;
mod token;

#[derive(Debug, Clone)]
pub enum Score<'s> {
    Note(std::collections::HashMap<&'s str, f64>),
    Row(Vec<Score<'s>>),
    Column(Vec<Score<'s>>),
}

impl<'s> Score<'s> {
    pub fn from_source(source: &'s str) -> Option<Score<'s>> {
        match lexer::lexer(source) {
            Ok(tokens) => match parser::parse(&mut tokens.iter()) {
                Ok(maps) => match compiler::compile(&maps) {
                    Ok(score) => Some(score),
                    Err(err) => {
                        println!("{}", err);
                        None
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    None
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
}