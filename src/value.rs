use crate::sound::Sound;

#[derive(Debug)]
pub enum Value {
    Real(f64),
    Bool(bool),
    Sound(Box<dyn Sound>),
}
