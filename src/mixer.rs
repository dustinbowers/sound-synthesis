pub trait SampleProvider {
    fn get_sample(&mut self) -> f32;
}
pub struct AudioMixer {
    channels: Vec<Box<dyn SampleProvider + Send>>,
}

impl AudioMixer {
    pub fn new() -> Self {
        Self { channels: vec![] }
    }

    pub fn add_channel(&mut self, channel: Box<dyn SampleProvider + Send>) {
        self.channels.push(channel);
    }

    pub fn get_mixed_sample(&mut self) -> f32 {
        self.channels.iter_mut().map(|c| c.get_sample()).sum()
    }
}
