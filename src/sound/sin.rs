use super::{Sound, SoundIter};

#[derive(Debug, Clone)]
pub struct Sin {
    frequency: f64,
    phase: f64,
}

pub struct SinIter {
    first: (f64, f64),
    next: (f64, f64),
}

use std::f64::consts::TAU;

impl Sin {
    // new とか？要るなら
}
impl Sound for Sin {
    fn iter(&self, samplerate: f64) -> Box<dyn SoundIter> {
        let t = TAU * self.frequency / samplerate;
        Box::new(SinIter {
            first: (t.cos(), t.sin()),
            next: (self.phase.cos(), self.phase.sin()),
        })
    }
    fn shift(&self, t: f64) -> Box<dyn Sound> {
        Box::new(Sin {
            frequency: self.frequency,
            phase: TAU * t * self.frequency,
        })
    }
}

impl SoundIter for SinIter {
    fn next(&mut self) -> f64 {
        self.next = (
            self.next.0 * self.first.1 + self.next.1 * self.first.0,
            self.next.0 * self.first.0 - self.next.1 * self.first.1,
        );
        self.next.1
    }
}
