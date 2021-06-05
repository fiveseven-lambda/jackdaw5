use crate::error::Error;
use crate::sound::Sound;

#[derive(Clone, Debug)]
// Clone は，変数の値を map から get して使う際に必要（多分）
pub enum Value {
    Real(f64),
    Bool(bool),
    Sound(Sound),
    Fnc0(fn() -> f64),
    Fnc1(fn(f64) -> f64),
    Fnc2(fn(f64, f64) -> f64),
    Fnc(fn(Vec<Value>) -> Option<Value>),
}

// Value 同士の計算
// （演算子による足し算とか）は Value をムーブしていい

impl Value {
    pub fn typename(&self) -> &'static str {
        match self {
            Value::Real(_) => "real",
            Value::Bool(_) => "bool",
            Value::Sound(_) => "Sound",
            Value::Fnc0(_) => "real()",
            Value::Fnc1(_) => "real(real)",
            Value::Fnc2(_) => "real(real, real)",
            Value::Fnc(_) => "function",
        }
    }
}

pub fn sin(arguments: Vec<Value>) -> Option<Value> {
    match &arguments[..] {
        [Value::Real(frequency)] => Some(Value::Sound(Sound::Sin {
            frequency: *frequency,
            phase: 0.,
        })),
        [Value::Real(frequency), Value::Real(phase)] => Some(Value::Sound(Sound::Sin {
            frequency: *frequency,
            phase: *phase,
        })),
        _ => None,
    }
}
