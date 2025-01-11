use hound::{SampleFormat, WavSpec, WavWriter};
use rand::{thread_rng, Rng};
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::{BufWriter, Result};
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    let mut random_audio: RandomAudioStream =
        RandomAudioStream::new(sample_rate, channels, duration);

    // Set up audio output device
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("Failed to get default output device");

    // Create a WAV writer
    let spec: WavSpec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer: WavWriter<BufWriter<File>> =
        WavWriter::create("./output.wav", spec).expect("Failed to create WAV file");

    // Buffer size for playback (chunk of samples)
    let buffer_size: usize = sample_rate as usize / 10; // Buffer 1/10th of a second

    let mut sample_buffer: Vec<i16> = Vec::with_capacity(buffer_size * channels as usize);

    // Synchronize playback start time
    let playback_start: Instant = Instant::now();

    // Stream the audio to both playback and WAV file
    while let Some(sample) = random_audio.next() {
        // Add sample to buffer
        sample_buffer.push(sample);

        // Write each sample to the WAV file
        writer.write_sample(sample).expect("Failed to write sample");

        // When buffer is full, play it
        if sample_buffer.len() >= buffer_size * channels as usize {
            let playback_buffer: SamplesBuffer<i16> =
                SamplesBuffer::new(channels, sample_rate, sample_buffer.clone());
            stream_handle
                .play_raw(playback_buffer.convert_samples())
                .expect("Failed to play audio stream");
            sample_buffer.clear();

            // Synchronize playback with real time
            let elapsed: Duration = playback_start.elapsed();
            let expected_elapsed: Duration = Duration::from_secs_f64(
                random_audio.samples_generated as f64 / (sample_rate as f64 * channels as f64),
            );

            if elapsed < expected_elapsed {
                sleep(expected_elapsed - elapsed);
            }
        }
    }

    // Finalize the WAV file
    writer.finalize().expect("Failed to finalize WAV file");

    Ok(())
}
