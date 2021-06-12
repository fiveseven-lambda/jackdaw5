mod error;
mod lexer;
mod pos;
mod token;
mod ast;
mod parser;
mod sound;
mod value;
mod function;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);

    let mut variables = std::collections::HashMap::new();
    variables.insert("sin".to_string(), value::Value::real_function_1(f64::sin));
    variables.insert("cos".to_string(), value::Value::real_function_1(f64::cos));
    variables.insert("tan".to_string(), value::Value::real_function_1(f64::tan));
    variables.insert("ln".to_string(), value::Value::real_function_1(f64::ln));
    variables.insert("log".to_string(), value::Value::real_function_1(f64::log10));
    variables.insert("max".to_string(), value::Value::real_function_2(f64::max));
    variables.insert("min".to_string(), value::Value::real_function_2(f64::min));
    variables.insert("E".to_string(), value::Value::Real(std::f64::consts::E));
    variables.insert("PI".to_string(), value::Value::Real(std::f64::consts::PI));
    variables.insert("True".to_string(), value::Value::Bool(true));
    variables.insert("False".to_string(), value::Value::Bool(false));
    variables.insert("Sin".to_string(), value::Value::Function(std::rc::Rc::new(function::Sin::new())));
    variables.insert("Exp".to_string(), value::Value::Function(std::rc::Rc::new(function::Exp::new())));
    variables.insert("Rand".to_string(), value::Value::Sound(sound::Sound::Rand));

    loop {
        match parser::parse_expression(&mut lexer) {
            Ok(Some(expression)) => {
                // println!("{:#?}", expression);
                match expression.evaluate(&variables) {
                    Some(Ok(value)) => println!("{:?}", value),
                    Some(Err(err)) => println!("{}", err),
                    None => println!("empty statement"),
                }
            }
            Ok(None) => break,
            Err(err) => break println!("{}", err),
        }
    }
}
