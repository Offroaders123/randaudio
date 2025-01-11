mod saw_wave_stream;

use crate::saw_wave_stream::SawWaveStream;
use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::{BufWriter, Result};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let sample_rate: u32 = 44100; // Standard audio sample rate
    let channels: u16 = 2; // Stereo
    let duration: Duration = Duration::from_secs(10); // Play for 10 seconds
    let frequency: f32 = 440.0; // Frequency of the saw wave (e.g., A4 note)
    let amplitude: f32 = 0.8 * (i16::MAX as f32); // Scale amplitude to fit i16 range

    // Create a random audio stream
    let random_audio: SawWaveStream =
        SawWaveStream::new(sample_rate, channels, duration, frequency, amplitude);

    // Set up audio output device for playback
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("Failed to get default output device");

    // Play the audio stream
    stream_handle
        .play_raw(random_audio.convert_samples())
        .expect("Failed to play audio stream");

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

    // Write the samples from RandomAudioStream to the WAV file
    let random_audio_for_file: SawWaveStream =
        SawWaveStream::new(sample_rate, channels, duration, frequency, amplitude);
    for sample in random_audio_for_file {
        writer.write_sample(sample).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");

    // Keep the program running until playback finishes
    sleep(duration);

    Ok(())
}
