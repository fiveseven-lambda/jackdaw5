#[derive(Clone, Debug)]
pub enum Sound {
    Const(f64),
    Linear { slope: f64, intercept: f64 },    // x = at + b
    Sin { frequency: f64, phase: f64 },       // x = sin(τft + θ)
    Exp { coefficient: f64, intercept: f64 }, // x = ae^(bt)
    Rand,
    Minus(Box<Sound>),
    Reciprocal(Box<Sound>),
    Add(Box<Sound>, Box<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Mul(Box<Sound>, Box<Sound>),
    Div(Box<Sound>, Box<Sound>),
    Pow(Box<Sound>, Box<Sound>),
}

use std::f64::consts::TAU;

impl Sound {
    pub fn shift(self, t: f64) -> Self {
        match self {
            Sound::Const(value) => Sound::Const(value),
            Sound::Linear {slope, intercept} => Sound::Linear {
                slope,
                intercept: slope * t + intercept,
            },
            Sound::Sin { frequency, phase } => Sound::Sin {
                frequency,
                phase: TAU * frequency * t + phase,
            },
            Sound::Exp { coefficient, intercept } => Sound::Exp {
                coefficient,
                intercept: intercept * (coefficient * t).exp()
            },
            Sound::Rand => Sound::Rand,
            Sound::Minus(sound) => Sound::Minus(sound.shift(t).into()),
            Sound::Reciprocal(sound) => Sound::Reciprocal(sound.shift(t).into()),
            Sound::Add(left, right) => Sound::Add(left.shift(t).into(), right.shift(t).into()),
            Sound::Sub(left, right) => Sound::Sub(left.shift(t).into(), right.shift(t).into()),
            Sound::Mul(left, right) => Sound::Mul(left.shift(t).into(), right.shift(t).into()),
            Sound::Div(left, right) => Sound::Div(left.shift(t).into(), right.shift(t).into()),
            Sound::Pow(left, right) => Sound::Pow(left.shift(t).into(), right.shift(t).into()),
        }
    }
    pub fn iter(self, samplerate: f64) -> SoundIter {
        match self {
            Sound::Const(value) => SoundIter::Const(value),
            Sound::Linear { slope, intercept } => SoundIter::Linear {
                next: intercept,
                difference: slope / samplerate,
            },
            Sound::Sin { frequency, phase } => SoundIter::Sin {
                next: Complex64::from_polar(1., phase),
                ratio: Complex64::from_polar(1., TAU * frequency / samplerate),
            },
            Sound::Exp { coefficient, intercept } => SoundIter::Exp {
                next: intercept,
                ratio: (coefficient / samplerate).exp(),
            },
            Sound::Rand => SoundIter::Rand(rand::thread_rng()),
            Sound::Minus(sound) => SoundIter::Minus(sound.iter(samplerate).into()),
            Sound::Reciprocal(sound) => SoundIter::Reciprocal(sound.iter(samplerate).into()),
            Sound::Add(left, right) => SoundIter::Add(left.iter(samplerate).into(), right.iter(samplerate).into()),
            Sound::Sub(left, right) => SoundIter::Sub(left.iter(samplerate).into(), right.iter(samplerate).into()),
            Sound::Mul(left, right) => SoundIter::Mul(left.iter(samplerate).into(), right.iter(samplerate).into()),
            Sound::Div(left, right) => SoundIter::Div(left.iter(samplerate).into(), right.iter(samplerate).into()),
            Sound::Pow(left, right) => SoundIter::Pow(left.iter(samplerate).into(), right.iter(samplerate).into()),
        }
    }
}

use num::complex::Complex64;
use rand::prelude::*;

pub enum SoundIter {
    Const(f64),
    Linear { next: f64, difference: f64 },
    Exp { next: f64, ratio: f64 },
    Sin { next: Complex64, ratio: Complex64 },
    Rand(ThreadRng),
    Minus(Box<SoundIter>),
    Reciprocal(Box<SoundIter>),
    Add(Box<SoundIter>, Box<SoundIter>),
    Sub(Box<SoundIter>, Box<SoundIter>),
    Mul(Box<SoundIter>, Box<SoundIter>),
    Div(Box<SoundIter>, Box<SoundIter>),
    Pow(Box<SoundIter>, Box<SoundIter>),
}

impl SoundIter {
    pub fn next(&mut self) -> f64 {
        match self {
            SoundIter::Const(value) => *value,
            SoundIter::Linear { next, difference } => {
                let ret = *next;
                *next += *difference;
                ret
            }
            SoundIter::Sin { next, ratio } => {
                let ret = next.im;
                *next *= *ratio;
                ret
            }
            SoundIter::Exp { next, ratio } => {
                let ret = *next;
                *next *= *ratio;
                ret
            }
            SoundIter::Rand(rng) => rng.gen(),
            SoundIter::Minus(iter) => -iter.next(),
            SoundIter::Reciprocal(iter) => 1. / iter.next(),
            SoundIter::Add(left, right) => left.next() + right.next(),
            SoundIter::Sub(left, right) => left.next() - right.next(),
            SoundIter::Mul(left, right) => left.next() * right.next(),
            SoundIter::Div(left, right) => left.next() / right.next(),
            SoundIter::Pow(left, right) => left.next().powf(right.next()),
        }
    }
}
