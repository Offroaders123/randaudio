use rand::{thread_rng, Rng};
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

struct RandomAudioStream {
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    samples_generated: usize,
}

impl RandomAudioStream {
    fn new(sample_rate: u32, channels: u16, duration: Duration) -> Self {
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

fn main() -> Result<()> {
    let sample_rate: u32 = 44100; // Standard audio sample rate
    let channels: u16 = 2; // Stereo
    let duration: Duration = Duration::from_secs(10); // Play for 10 seconds

    // Create a random audio stream
    let random_audio: RandomAudioStream = RandomAudioStream::new(sample_rate, channels, duration);

    // Set up audio output device
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("Failed to get default output device");

    // Play the audio stream
    stream_handle
        .play_raw(random_audio.convert_samples())
        .expect("Failed to play audio stream");

    // Keep the program running until playback finishes
    sleep(duration);

    Ok(())
}
