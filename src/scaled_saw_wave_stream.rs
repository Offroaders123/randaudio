use rodio::Source;
use std::time::Duration;

pub struct ScaledSawWaveStream {
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    samples_generated: usize,
    modulation_frequency: f32, // Frequency of modulation (roller coaster speed)
    modulation_depth: f32,     // How much the frequency changes up and down
    amplitude: f32,            // Amplitude of the wave
    phase: f32,                // Current phase of the wave
    scale: Vec<f32>,           // Major scale frequencies
    current_note_index: usize, // Index to track the current note in the scale
}

impl ScaledSawWaveStream {
    pub fn new(
        sample_rate: u32,
        channels: u16,
        duration: Duration,
        base_frequency: f32,
        modulation_frequency: f32,
        modulation_depth: f32,
        amplitude: f32,
    ) -> Self {
        // Define the frequencies for the major scale based on the base_frequency (A4 = 440 Hz)
        let scale: Vec<f32> = vec![
            base_frequency,         // Root (e.g., A4 = 440 Hz)
            base_frequency * 1.125, // Major second
            base_frequency * 1.25,  // Major third
            base_frequency * 1.5,   // Perfect fourth
            base_frequency * 1.75,  // Perfect fifth
            base_frequency * 2.0,   // Major sixth
            base_frequency * 2.25,  // Major seventh
            base_frequency * 2.5,   // Octave
        ];

        ScaledSawWaveStream {
            sample_rate,
            channels,
            duration,
            samples_generated: 0,
            modulation_frequency,
            modulation_depth,
            amplitude,
            phase: 0.0,
            scale,
            current_note_index: 0,
        }
    }

    // Helper function to snap to the nearest note in the scale
    fn snap_to_nearest_note(&self, frequency: f32) -> f32 {
        // Find the closest frequency in the scale
        self.scale
            .iter()
            .min_by(|a, b| {
                (*a - frequency)
                    .abs()
                    .partial_cmp(&(*b - frequency).abs())
                    .unwrap()
            })
            .cloned()
            .unwrap_or(frequency) // Default to the input frequency if no close match is found
    }

    // Helper function to move to the next note in the scale
    fn next_note_frequency(&mut self) -> f32 {
        // Move to the next note in the scale and wrap around if necessary
        self.current_note_index = (self.current_note_index + 1) % self.scale.len();
        self.scale[self.current_note_index]
    }
}

impl Iterator for ScaledSawWaveStream {
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

        // Get the current note frequency from the scale
        let frequency: f32 = self.next_note_frequency();

        // Calculate the modulated frequency and snap it to the nearest note
        let modulated_frequency: f32 =
            self.snap_to_nearest_note(frequency + modulation * self.modulation_depth);

        // Generate a saw wave value by incrementing the phase based on the modulated frequency
        let sample: i16 = (self.phase * self.amplitude) as i16;

        // Increment the phase to simulate the saw wave pattern, modulated by the changing frequency
        self.phase += modulated_frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        Some(sample)
    }
}

impl Source for ScaledSawWaveStream {
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
