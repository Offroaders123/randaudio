mod scaled_saw_wave_stream;

use crate::scaled_saw_wave_stream::ScaledSawWaveStream;
use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::{BufWriter, Result};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let write_file: bool = true;

    let sample_rate: u32 = 44100; // Standard audio sample rate
    let channels: u16 = 2; // Stereo
    let duration: Duration = Duration::from_secs(10); // Play for 10 seconds
    let base_frequency: f32 = 60.0; // Base frequency of the saw wave (e.g., A4 note)
    let modulation_frequency: f32 = 4.0; // How fast the frequency fluctuates (lower = slower)
    let modulation_depth: f32 = 40.0; // How much the frequency varies up and down (higher = more dramatic)
    let amplitude: f32 = 0.8 * (i16::MAX as f32); // Scale amplitude to fit i16 range

    // Create a saw wave audio stream
    let random_audio: ScaledSawWaveStream = ScaledSawWaveStream::new(
        sample_rate,
        channels,
        duration,
        base_frequency,
        modulation_frequency,
        modulation_depth,
        amplitude,
    );

    // Set up audio output device for playback
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("Failed to get default output device");

    // Play the audio stream
    stream_handle
        .play_raw(random_audio.convert_samples())
        .expect("Failed to play audio stream");

    if write_file {
        write_file_from_stream(
            sample_rate,
            channels,
            duration,
            base_frequency,
            modulation_frequency,
            modulation_depth,
            amplitude,
        );
    }

    // Keep the program running until playback finishes
    sleep(duration);

    Ok(())
}

fn write_file_from_stream(
    sample_rate: u32,
    channels: u16,
    duration: Duration,
    base_frequency: f32,
    modulation_frequency: f32,
    modulation_depth: f32,
    amplitude: f32,
) {
    // Create WAV file
    let spec: WavSpec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    // Open the output WAV file
    let mut writer: WavWriter<BufWriter<File>> =
        WavWriter::create("output.wav", spec).expect("Failed to create WAV file");

    // Write the samples from the audio stream to the WAV file
    let random_audio_for_file: ScaledSawWaveStream = ScaledSawWaveStream::new(
        sample_rate,
        channels,
        duration,
        base_frequency,
        modulation_frequency,
        modulation_depth,
        amplitude,
    );
    for sample in random_audio_for_file {
        writer.write_sample(sample).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");
}
