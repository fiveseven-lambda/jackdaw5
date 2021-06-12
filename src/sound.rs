#[derive(Clone, Debug)]
pub enum Sound {
    Const(f64),
    Linear { slope: f64, intercept: f64 },
    Sin { frequency: f64, phase: f64 },
    Exp { base: f64 },
    Rand,
    Minus(Box<Sound>),
    Reciprocal(Box<Sound>),
    Add(Box<Sound>, Box<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Mul(Box<Sound>, Box<Sound>),
    Div(Box<Sound>, Box<Sound>),
    Pow(Box<Sound>, Box<Sound>),
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

            Sound::Minus(sound) => SoundIter::Minus(sound.iter().into()),
            Sound::Reciprocal(sound) => SoundIter::Reciprocal(sound.iter().into()),
            Sound::Add(left, right) => SoundIter::Add(left.iter().into(), right.iter().into()),
            Sound::Sub(left, right) => SoundIter::Sub(left.iter().into(), right.iter().into()),
            Sound::Mul(left, right) => SoundIter::Mul(left.iter().into(), right.iter().into()),
            Sound::Div(left, right) => SoundIter::Div(left.iter().into(), right.iter().into()),
            Sound::Pow(left, right) => SoundIter::Pow(left.iter().into(), right.iter().into()),
            _ => todo!(),
        }
    }
}

pub enum SoundIter {
    Const(f64),
    Linear { prev: f64, difference: f64 },
    Exp { prev: f64, ratio: f64 },
    Sin { prev: (f64, f64), ratio: (f64, f64) },
    Minus(Box<SoundIter>),
    Reciprocal(Box<SoundIter>),
    Add(Box<SoundIter>, Box<SoundIter>),
    Sub(Box<SoundIter>, Box<SoundIter>),
    Mul(Box<SoundIter>, Box<SoundIter>),
    Div(Box<SoundIter>, Box<SoundIter>),
    Pow(Box<SoundIter>, Box<SoundIter>),
}
