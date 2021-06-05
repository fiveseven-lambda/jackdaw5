mod ast;
mod error;
mod lexer;
mod pos;
mod token;
mod parser;
mod value;
mod sound;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    let mut map = std::collections::HashMap::new();
    map.insert("sin".to_string(), value::Value::Fnc1(f64::sin));
    map.insert("cos".to_string(), value::Value::Fnc1(f64::cos));
    map.insert("tan".to_string(), value::Value::Fnc1(f64::tan));
    map.insert("exp".to_string(), value::Value::Fnc1(f64::exp));
    map.insert("ln".to_string(), value::Value::Fnc1(f64::ln));
    map.insert("log".to_string(), value::Value::Fnc1(f64::log10));
    map.insert("max".to_string(), value::Value::Fnc2(f64::max));
    map.insert("min".to_string(), value::Value::Fnc2(f64::min));
    map.insert("rand".to_string(), value::Value::Fnc0(rand));
    map.insert("PI".to_string(), value::Value::Real(std::f64::consts::PI));
    map.insert("E".to_string(), value::Value::Real(std::f64::consts::E));
    map.insert("Sin".to_string(), value::Value::Fnc(value::sin));
    loop {
        match parser::parse_expression(&mut lexer) {
            Ok(Some(expression)) => {
                // println!("{:?}", expression);
                match expression.evaluate(&mut map) {
                    Some(Ok(value)) => println!("{:?}", value),
                    Some(Err(err)) => println!("{}", err),
                    None => println!("empty sentence"),
                }
            }
            Ok(None) => break,
            Err(err) => break eprintln!("{}", err),
        }
    }
}

fn rand() -> f64 {
    use rand::Rng;
    rand::thread_rng().gen()
}
