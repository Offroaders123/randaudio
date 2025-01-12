mod file_audio_stream;

use crate::file_audio_stream::FileAudioStream;
use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::{BufWriter, Result};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let write_file: bool = false;

    let file: &str = "/Applications/GarageBand.app/Contents/MacOS/GarageBand";
    let sample_rate: u32 = 44100; // Standard audio sample rate
    let channels: u16 = 2; // Stereo
    let duration: Duration = Duration::from_secs(100); // Play for 10 seconds

    // Create a saw wave audio stream
    let random_audio: FileAudioStream = FileAudioStream::new(file, sample_rate, channels)?;

    // Set up audio output device for playback
    let (_stream, stream_handle): (OutputStream, OutputStreamHandle) =
        OutputStream::try_default().expect("Failed to get default output device");

    // Play the audio stream
    stream_handle
        .play_raw(random_audio.convert_samples())
        .expect("Failed to play audio stream");

    if write_file {
        write_file_from_stream(file, sample_rate, channels)?;
    }

    // Keep the program running until playback finishes
    sleep(duration);

    Ok(())
}

fn write_file_from_stream(file: &str, sample_rate: u32, channels: u16) -> Result<()> {
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
    let random_audio_for_file: FileAudioStream = FileAudioStream::new(file, sample_rate, channels)?;
    for sample in random_audio_for_file {
        writer.write_sample(sample).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");

    Ok(())
}
