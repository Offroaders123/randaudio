use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

// Entry point for WASM
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set up the Web Audio API context
    let audio_context: AudioContext = AudioContext::new()?;

    // Generate random audio samples
    let sample_rate: u32 = 44100;
    let duration_secs: u32 = 5;
    let total_samples: u32 = sample_rate * duration_secs;
    let mut samples: Vec<f32> = Vec::with_capacity(total_samples as usize);

    for _ in 0..total_samples {
        samples.push(rand::thread_rng().gen_range(-1.0..1.0)); // Generate random values between -1.0 and 1.0
    }

    // Convert samples to an AudioBuffer
    let audio_buffer: AudioBuffer =
        AudioContext::create_buffer(&audio_context, 1, total_samples, sample_rate as f32)?;
    let mut channel_data: Vec<f32> = audio_buffer.get_channel_data(0)?;
    channel_data.copy_from_slice(&samples);

    // Create a buffer source and connect it to the destination
    let buffer_source: AudioBufferSourceNode = audio_context.create_buffer_source()?;
    buffer_source.set_buffer(Some(&audio_buffer));
    buffer_source.connect_with_audio_node(&audio_context.destination())?;
    buffer_source.start()?;

    Ok(())
}
