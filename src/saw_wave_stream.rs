use rodio::Source;
use std::time::Duration;

pub struct SawWaveStream {
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    samples_generated: usize,
    base_frequency: f32,       // Base frequency of the saw wave
    modulation_frequency: f32, // Frequency of modulation (roller coaster speed)
    modulation_depth: f32,     // How much the frequency changes up and down
    amplitude: f32,            // Amplitude of the wave
    phase: f32,                // Current phase of the wave
}

impl SawWaveStream {
    pub fn new(
        sample_rate: u32,
        channels: u16,
        duration: Duration,
        base_frequency: f32,
        modulation_frequency: f32,
        modulation_depth: f32,
        amplitude: f32,
    ) -> Self {
        SawWaveStream {
            sample_rate,
            channels,
            duration,
            samples_generated: 0,
            base_frequency,
            modulation_frequency,
            modulation_depth,
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

        // Calculate the modulation factor: a sine wave oscillating between -1 and 1
        let modulation: f32 = (self.modulation_frequency * self.samples_generated as f32
            / self.sample_rate as f32)
            .sin();

        // Calculate the new frequency by adding the modulation depth to the base frequency
        let frequency: f32 = self.base_frequency + modulation * self.modulation_depth;

        // Generate a saw wave value by incrementing the phase based on the modulated frequency
        let sample: i16 = (self.phase * self.amplitude) as i16;

        // Increment the phase to simulate the saw wave pattern, modulated by the changing frequency
        self.phase += frequency / self.sample_rate as f32;
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
