use crate::sound::Sound;
use crate::value::Value;
use std::cell::Cell;

pub enum Argument<'cell> {
    Real(&'cell Cell<f64>),
    Bool(&'cell Cell<bool>),
    Sound(&'cell Cell<Sound>),
    String(&'cell Cell<String>),
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

pub struct Sin(Cell<f64>);
impl Sin {
    pub fn new() -> Sin {
        Sin(Cell::new(0.))
    }
}
impl Function for Sin {
    fn arguments(&self) -> Vec<Argument> {
        vec![Argument::Real(&self.0)]
    }
    fn invoke(&self) -> Value {
        Value::Sound(Sound::Sin {
            frequency: self.0.get(),
            phase: 0.,
        })
    }
}

pub struct Exp(Cell<f64>);
impl Exp {
    pub fn new() -> Exp {
        Exp(Cell::new(0.))
    }
}
impl Function for Exp {
    fn arguments(&self) -> Vec<Argument> {
        vec![Argument::Real(&self.0)]
    }
    fn invoke(&self) -> Value {
        let tau = self.0.get(); // 時定数
        Value::Sound(Sound::Exp {
            coefficient: -1. / tau,
            intercept: 1.,
        })
    }
}
