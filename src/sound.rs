pub trait Sound: std::fmt::Debug {
    fn shift(&self, t: f64) -> Box<dyn Sound>;
    fn iter(&self, samplerate: f64) -> Box<dyn SoundIter>;
}

pub trait SoundIter {
    fn next(&mut self) -> f64;
}

mod sin;
