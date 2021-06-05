#[derive(Clone, Debug)]
pub enum Sound {
    Const(f64),
    Linear {
        slope: f64,
        intercept: f64,
    },
    Sin {
        frequency: f64,
        phase: f64,
    },
    Exp {
        base: f64,
    },
    Rand,
    Add(Box<Sound>, Box<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Mul(Box<Sound>, Box<Sound>),
    Div(Box<Sound>, Box<Sound>),
    Fnc0 {
        f: fn() -> f64,
    },
    Fnc1 {
        f: fn(f64) -> f64,
        arg: Box<Sound>,
    },
    Fnc2 {
        f: fn(f64, f64) -> f64,
        arg1: Box<Sound>,
        arg2: Box<Sound>,
    },
}

impl Sound {
    pub fn shift(self, t: f64) -> Self {
        match self {
            Sound::Const(value) => Sound::Const(value),
            _ => todo!(),
        }
    }
    pub fn iter(self) -> SoundIter {
        match self {
            Sound::Const(value) => SoundIter::Const(value),

            Sound::Add(left, right) => SoundIter::Add(left.iter().into(), right.iter().into()),
            _ => todo!(),
        }
    }
}

pub enum SoundIter {
    Const(f64),
    Linear { prev: f64, difference: f64 },
    Exp { prev: f64, ratio: f64 },
    Sin { prev: (f64, f64), ratio: (f64, f64) },
    Add(Box<SoundIter>, Box<SoundIter>),
}
