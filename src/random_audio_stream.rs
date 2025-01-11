use rand::{thread_rng, Rng};
use rodio::Source;
use std::time::Duration;

pub struct RandomAudioStream {
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    samples_generated: usize,
}

impl RandomAudioStream {
    pub fn new(sample_rate: u32, channels: u16, duration: Duration) -> Self {
        RandomAudioStream {
            sample_rate,
            channels,
            duration,
            samples_generated: 0,
        }
    }
}

impl Iterator for RandomAudioStream {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop generating samples after the intended duration
        let total_samples: usize =
            self.sample_rate as usize * self.duration.as_secs() as usize * self.channels as usize;

        if self.samples_generated >= total_samples {
            return None;
        }

        self.samples_generated += 1;

        // Generate a random i16 value for audio data
        Some(thread_rng().gen_range(i16::MIN..=i16::MAX))
    }
}

impl Source for RandomAudioStream {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}
