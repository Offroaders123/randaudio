use rodio::Source;
use std::fs::File;
use std::io::{Read, Result};
use std::time::Duration;

pub struct FileAudioStream {
    sample_rate: u32,
    channels: u16,
    data: Vec<i16>,
    position: usize,
    duration: Duration,
}

impl FileAudioStream {
    pub fn new(file_path: &str, sample_rate: u32, channels: u16) -> Result<Self> {
        // Read the file's bytes
        let mut file: File = File::open(file_path)?;
        let mut raw_data: Vec<u8> = Vec::new();
        file.read_to_end(&mut raw_data)?;

        // Interpret the file bytes as `i16` samples
        let data: Vec<i16> = raw_data
            .chunks_exact(2)
            .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
            .collect();

        // Estimate the duration based on the number of samples
        let duration: Duration =
            Duration::from_secs_f32(data.len() as f32 / (sample_rate as f32 * channels as f32));

        Ok(FileAudioStream {
            sample_rate,
            channels,
            data,
            position: 0,
            duration,
        })
    }
}

impl Iterator for FileAudioStream {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.data.len() {
            let sample: i16 = self.data[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for FileAudioStream {
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
