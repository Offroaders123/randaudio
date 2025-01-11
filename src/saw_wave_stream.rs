use rodio::Source;
use std::time::Duration;

pub struct SawWaveStream {
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    samples_generated: usize,
    frequency: f32, // Frequency of the saw wave
    amplitude: f32, // Amplitude of the wave
    phase: f32,     // Current phase of the wave
}

impl SawWaveStream {
    pub fn new(
        sample_rate: u32,
        channels: u16,
        duration: Duration,
        frequency: f32,
        amplitude: f32,
    ) -> Self {
        SawWaveStream {
            sample_rate,
            channels,
            duration,
            samples_generated: 0,
            frequency,
            amplitude,
            phase: 0.0,
        }
    }
}

impl Iterator for SawWaveStream {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop generating samples after the intended duration
        let total_samples: usize =
            self.sample_rate as usize * self.duration.as_secs() as usize * self.channels as usize;

        if self.samples_generated >= total_samples {
            return None;
        }

        self.samples_generated += 1;

        // Generate a saw wave value by incrementing the phase
        let sample: i16 = (self.phase * self.amplitude) as i16;

        // Increment the phase to simulate the saw wave pattern
        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        Some(sample)
    }
}

impl Source for SawWaveStream {
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
