use std::sync::{Arc, Mutex};
use tinyaudio::{run_output_device, BaseAudioOutputDevice, OutputDeviceParameters};

use crate::mixer;

pub fn init_audio(
    global_mixer: &Arc<Mutex<mixer::AudioMixer>>,
) -> Option<Box<dyn BaseAudioOutputDevice>> {
    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: 44100,
        channel_sample_count: 735,
    };

    let mixer_clone = Arc::clone(&global_mixer);
    let device = run_output_device(params, {
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                for sample in samples {
                    let mut mixer = mixer_clone.lock().unwrap();
                    *sample = mixer.get_mixed_sample();
                }
            }
        }
    });

    match device {
        Ok(d) => Some(d),
        Err(_) => None,
    }
}
