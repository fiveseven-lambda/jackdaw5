use crate::sound::Sound;

use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Real(f64),
    Bool(bool),
    Sound(Sound),
    Function(Rc<dyn Function>),
    RealFunction(Rc<dyn RealFunction>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Real(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Sound(value) => write!(f, "{:?}", value),
            Value::Function(_) => write!(f, "function"),
            Value::RealFunction(_) => write!(f, "real function"),
        }
    }
}

use std::cell::Cell;

pub enum Argument<'cell> {
    Real(&'cell Cell<f64>),
    Bool(&'cell Cell<bool>),
    Sound(&'cell Cell<Sound>),
}

pub trait Function {
    fn arguments(&self) -> Vec<Argument>;
    fn invoke(&self) -> Value;
}
pub trait RealFunction {
    fn arguments(&self) -> Vec<Argument>;
    fn invoke(&self) -> f64;
}

pub struct PrimitiveRealFunction1(fn(f64) -> f64, Cell<f64>);
impl PrimitiveRealFunction1 {
    pub fn new(fnc: fn(f64) -> f64) -> PrimitiveRealFunction1 {
        PrimitiveRealFunction1(fnc, Cell::new(0.))
    }
}
impl RealFunction for PrimitiveRealFunction1 {
    fn arguments(&self) -> Vec<Argument> {
        vec![Argument::Real(&self.1)]
    }
    fn invoke(&self) -> f64 {
        self.0(self.1.get())
    }
}

pub struct PrimitiveRealFunction2(fn(f64, f64) -> f64, Cell<f64>, Cell<f64>);
impl PrimitiveRealFunction2 {
    pub fn new(fnc: fn(f64, f64) -> f64) -> PrimitiveRealFunction2 {
        PrimitiveRealFunction2(fnc, Cell::new(0.), Cell::new(0.))
    }
}
impl RealFunction for PrimitiveRealFunction2 {
    fn arguments(&self) -> Vec<Argument> {
        vec![Argument::Real(&self.1), Argument::Real(&self.2)]
    }
    fn invoke(&self) -> f64 {
        self.0(self.1.get(), self.2.get())
    }
}
