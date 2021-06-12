use crate::function::{Function, PrimitiveRealFunction1, PrimitiveRealFunction2, RealFunction};
use crate::sound::Sound;

use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Real(f64),
    Bool(bool),
    Sound(Sound),
    String(String),
    Function(Rc<dyn Function>),
    RealFunction(Rc<dyn RealFunction>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Real(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Sound(value) => write!(f, "{:?}", value),
            Value::Function(_) => write!(f, "function"),
            Value::RealFunction(_) => write!(f, "real function"),
        }
    }
}

impl Value {
    pub fn real_function_1(f: fn(f64) -> f64) -> Value {
        Value::RealFunction(Rc::new(PrimitiveRealFunction1::new(f.into())))
    }
    pub fn real_function_2(f: fn(f64, f64) -> f64) -> Value {
        Value::RealFunction(Rc::new(PrimitiveRealFunction2::new(f.into())))
    }
}
