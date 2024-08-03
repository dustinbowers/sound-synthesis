use crate::mixer::SampleProvider;

#[derive(Debug)]
pub struct SineWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub sample_rate: f32,
}

impl SampleProvider for SineWave {
    fn get_sample(&mut self) -> f32 {
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude;
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}

#[derive(Debug)]
pub struct SquareWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub sample_rate: f32,
}

impl SampleProvider for SquareWave {
    fn get_sample(&mut self) -> f32 {
        let sample = if self.phase < 0.5 {
            self.amplitude
        } else {
            -self.amplitude
        };
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}

#[derive(Debug)]
pub struct TriangleWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub sample_rate: f32,
}

impl SampleProvider for TriangleWave {
    fn get_sample(&mut self) -> f32 {
        let sample = if self.phase < 0.5 {
            self.amplitude * (self.phase * 4.0 - 1.0)
        } else {
            self.amplitude * (3.0 - self.phase * 4.0)
        };
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}

// TODO
//#[derive(Debug)]
// pub struct Noise {}
