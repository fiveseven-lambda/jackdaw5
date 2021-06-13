use crate::sound::Sound;
use crate::value::Value;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Argument {
    Real(Rc<Cell<f64>>),
    Boolean(Rc<Cell<bool>>),
    Sound(Rc<Cell<Sound>>),
    String(Rc<Cell<String>>),
}

impl Argument {
    fn type_name(&self) -> &'static str {
        match self {
            Argument::Real(_) => "real",
            Argument::Boolean(_) => "boolean",
            Argument::Sound(_) => "Sound",
            Argument::String(_) => "string",
        }
    }
    pub fn set(&self, value: Value) -> Result<(), (&'static str, Value)> {
        match (self, value) {
            (Argument::Real(cell), Value::Real(value)) => cell.set(value),
            (Argument::Boolean(cell), Value::Boolean(value)) => cell.set(value),
            (Argument::Sound(cell), Value::Sound(value)) => cell.set(value),
            (Argument::String(cell), Value::String(value)) => cell.set(value),
            (_, value) => return Err((self.type_name(), value)),
        };
        Ok(())
    }
}

pub trait Function {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>);
    fn invoke(&self) -> Value;
}
pub trait RealFunction {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>);
    fn invoke(&self) -> f64;
}

pub struct PrimitiveRealFunction1(fn(f64) -> f64, Rc<Cell<f64>>);
impl PrimitiveRealFunction1 {
    pub fn new(fnc: fn(f64) -> f64) -> PrimitiveRealFunction1 {
        PrimitiveRealFunction1(fnc, Rc::new(Cell::new(0.)))
    }
}
impl RealFunction for PrimitiveRealFunction1 {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>) {
        (vec![Argument::Real(self.1.clone())], HashMap::new())
    }
    fn invoke(&self) -> f64 {
        self.0(self.1.get())
    }
}

pub struct PrimitiveRealFunction2(fn(f64, f64) -> f64, Rc<Cell<f64>>, Rc<Cell<f64>>);
impl PrimitiveRealFunction2 {
    pub fn new(fnc: fn(f64, f64) -> f64) -> PrimitiveRealFunction2 {
        PrimitiveRealFunction2(fnc, Rc::new(Cell::new(0.)), Rc::new(Cell::new(0.)))
    }
}
impl RealFunction for PrimitiveRealFunction2 {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>) {
        (vec![Argument::Real(self.1.clone()), Argument::Real(self.2.clone())], HashMap::new())
    }
    fn invoke(&self) -> f64 {
        self.0(self.1.get(), self.2.get())
    }
}

pub struct Sin(Rc<Cell<f64>>);
impl Sin {
    pub fn new() -> Sin {
        Sin(Rc::new(Cell::new(0.)))
    }
}
impl Function for Sin {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>) {
        (vec![Argument::Real(self.0.clone())], HashMap::new())
    }
    fn invoke(&self) -> Value {
        Value::Sound(Sound::Sin {
            frequency: self.0.get(),
            phase: 0.,
        })
    }
}

pub struct Exp(Rc<Cell<f64>>);
impl Exp {
    pub fn new() -> Exp {
        Exp(Rc::new(Cell::new(0.)))
    }
}
impl Function for Exp {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>) {
        (vec![Argument::Real(self.0.clone())], HashMap::new())
    }
    fn invoke(&self) -> Value {
        let tau = self.0.get(); // 時定数
        Value::Sound(Sound::Exp {
            coefficient: -1. / tau,
            intercept: 1.,
        })
    }
}

pub struct Linear {
    x0: Rc<Cell<f64>>,
    x1: Rc<Cell<f64>>,
    t1: Rc<Cell<f64>>,
}
impl Linear {
    pub fn new() -> Linear {
        Linear {
            x0: Rc::new(Cell::new(0.)),
            x1: Rc::new(Cell::new(0.)),
            t1: Rc::new(Cell::new(0.)),
        }
    }
}
impl Function for Linear {
    fn arguments(&self) -> (Vec<Argument>, HashMap<String, Argument>) {
        self.t1.set(1.);
        (
            vec![Argument::Real(self.x0.clone()), Argument::Real(self.x1.clone())],
            vec![("t1".to_string(), Argument::Real(self.t1.clone()))].into_iter().collect(),
        )
    }
    fn invoke(&self) -> Value {
        let x0 = self.x0.get();
        let x1 = self.x1.get();
        let t1 = self.t1.get();
        Value::Sound(Sound::Linear {
            slope: (x1 - x0) / t1,
            intercept: x0,
        })
    }
}
