pub trait Sound {
    // : Clone とか？要るかも
    type Iter: SoundIter;
    fn shift(&self, t: f64) -> Self;
    fn iter(&self, samplerate: f64) -> Self::Iter;
}

pub trait SoundIter {
    fn next(&mut self) -> f64;
}

mod sin;
