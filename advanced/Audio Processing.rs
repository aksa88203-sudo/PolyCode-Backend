// audio_processing.rs
// Audio processing examples in Rust

use std::f32::consts::PI;

// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
    pub bit_depth: u16,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 512,
            bit_depth: 16,
        }
    }
}

// Wave generator
pub struct WaveGenerator {
    config: AudioConfig,
    phase: f32,
    frequency: f32,
    amplitude: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum WaveType {
    Sine, Square, Sawtooth, Triangle, Noise,
}

impl WaveGenerator {
    pub fn new(config: AudioConfig) -> Self {
        WaveGenerator {
            config,
            phase: 0.0,
            frequency: 440.0,
            amplitude: 0.5,
        }
    }
    
    pub fn generate_samples(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.amplitude * (self.phase).sin();
            self.advance_phase();
        }
    }
    
    fn advance_phase(&mut self) {
        let phase_increment = 2.0 * PI * self.frequency / self.config.sample_rate as f32;
        self.phase += phase_increment;
        while self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
    }
}

// Basic DSP processor
pub struct DspProcessor {
    config: AudioConfig,
    delay_line: Vec<f32>,
    delay_index: usize,
}

impl DspProcessor {
    pub fn new(config: AudioConfig) -> Self {
        DspProcessor {
            config,
            delay_line: vec![0.0; 44100],
            delay_index: 0,
        }
    }
    
    pub fn apply_delay(&mut self, input: &[f32], output: &mut [f32], delay_ms: f32) {
        let delay_samples = (delay_ms / 1000.0 * self.config.sample_rate as f32) as usize;
        
        for (i, &sample) in input.iter().enumerate() {
            let delayed_sample = self.delay_line[
                (self.delay_index + self.delay_line.len() - delay_samples) % self.delay_line.len()
            ];
            output[i] = sample + delayed_sample * 0.5;
            self.delay_line[self.delay_index] = sample;
            self.delay_index = (self.delay_index + 1) % self.delay_line.len();
        }
    }
}

// Audio effects
pub struct AudioEffects {
    config: AudioConfig,
}

impl AudioEffects {
    pub fn new(config: AudioConfig) -> Self {
        AudioEffects { config }
    }
    
    pub fn apply_distortion(input: &[f32], output: &mut [f32], drive: f32, mix: f32) {
        for (i, &sample) in input.iter().enumerate() {
            let driven = sample * drive;
            let distorted = if driven > 1.0 { 1.0 } else if driven < -1.0 { -1.0 } else { driven };
            output[i] = sample * (1.0 - mix) + distorted * mix;
        }
    }
    
    pub fn apply_gain(samples: &mut [f32], gain_db: f32) {
        let gain_linear = 10.0f32.powf(gain_db / 20.0);
        for sample in samples.iter_mut() {
            *sample *= gain_linear;
        }
    }
}

// Musical notes
pub struct MusicalNote {
    pub frequency: f32,
    pub name: &'static str,
}

impl MusicalNote {
    pub const A4: MusicalNote = MusicalNote { frequency: 440.0, name: "A4" };
    pub const C4: MusicalNote = MusicalNote { frequency: 261.63, name: "C4" };
    
    pub fn from_midi_note(midi_note: u8) -> MusicalNote {
        let a4_freq = 440.0;
        let a4_midi = 69;
        let freq = a4_freq * 2.0f32.powf((midi_note as f32 - a4_midi as f32) / 12.0);
        MusicalNote { frequency: freq, name: "MIDI" }
    }
}

// Audio utilities
pub fn convert_sample_rate(input_samples: &[f32], input_rate: u32, output_rate: u32) -> Vec<f32> {
    if input_rate == output_rate {
        return input_samples.to_vec();
    }
    
    let ratio = output_rate as f64 / input_rate as f64;
    let output_length = (input_samples.len() as f64 * ratio) as usize;
    let mut output = Vec::with_capacity(output_length);
    
    for i in 0..output_length {
        let input_index = i as f64 / ratio;
        let index_floor = input_index.floor() as usize;
        let index_ceil = (index_floor + 1).min(input_samples.len() - 1);
        let fraction = input_index - index_floor as f64;
        
        let sample = input_samples[index_floor] * (1.0 - fraction) as f32 +
                     input_samples[index_ceil] * fraction as f32;
        output.push(sample);
    }
    
    output
}

pub fn mono_to_stereo(mono_samples: &[f32]) -> Vec<f32> {
    let mut stereo = Vec::with_capacity(mono_samples.len() * 2);
    for &sample in mono_samples {
        stereo.push(sample);
        stereo.push(sample);
    }
    stereo
}

pub fn stereo_to_mono(stereo_samples: &[f32]) -> Vec<f32> {
    let mut mono = Vec::with_capacity(stereo_samples.len() / 2);
    for chunk in stereo_samples.chunks(2) {
        if chunk.len() == 2 {
            let mono_sample = (chunk[0] + chunk[1]) / 2.0;
            mono.push(mono_sample);
        }
    }
    mono
}

// Main demonstration
fn main() {
    println!("=== AUDIO PROCESSING DEMONSTRATIONS ===\n");
    
    let config = AudioConfig::default();
    println!("Audio config: {:?}", config);
    
    // Wave generation
    let mut generator = WaveGenerator::new(config.clone());
    let mut buffer = vec![0.0; 1024];
    generator.generate_samples(&mut buffer);
    println!("Generated {} audio samples", buffer.len());
    
    // DSP processing
    let mut processor = DspProcessor::new(config.clone());
    let mut output = vec![0.0; buffer.len()];
    processor.apply_delay(&buffer, &mut output, 100.0);
    println!("Applied delay effect to {} samples", output.len());
    
    // Audio effects
    let mut effected = vec![0.0; output.len()];
    AudioEffects::apply_distortion(&output, &mut effected, 2.0, 0.5);
    println!("Applied distortion effect");
    
    // Sample rate conversion
    let converted = convert_sample_rate(&buffer, 44100, 48000);
    println!("Converted {} samples from 44100Hz to 48000Hz", converted.len());
    
    // Channel conversion
    let stereo = mono_to_stereo(&buffer);
    let mono = stereo_to_mono(&stereo);
    println!("Converted {} samples mono->stereo->mono", mono.len());
    
    // Musical notes
    println!("Musical notes:");
    println!("  A4: {} Hz", MusicalNote::A4.frequency);
    println!("  C4: {} Hz", MusicalNote::C4.frequency);
    
    let midi_note = MusicalNote::from_midi_note(60);
    println!("  MIDI note 60: {} Hz", midi_note.frequency);
    
    // Gain adjustment
    let mut gain_buffer = buffer.clone();
    AudioEffects::apply_gain(&mut gain_buffer, -6.0);
    println!("Applied -6dB gain to {} samples", gain_buffer.len());
    
    println!("\n=== AUDIO PROCESSING DEMONSTRATIONS COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wave_generator() {
        let config = AudioConfig::default();
        let mut generator = WaveGenerator::new(config);
        let mut buffer = vec![0.0; 10];
        generator.generate_samples(&mut buffer);
        
        // Check that samples are within expected range
        for sample in buffer {
            assert!(sample >= -0.5 && sample <= 0.5);
        }
    }
    
    #[test]
    fn test_channel_conversion() {
        let mono = vec![1.0, 2.0, 3.0];
        let stereo = mono_to_stereo(&mono);
        assert_eq!(stereo.len(), 6);
        assert_eq!(stereo[0], 1.0);
        assert_eq!(stereo[1], 1.0);
        
        let back_to_mono = stereo_to_mono(&stereo);
        assert_eq!(back_to_mono.len(), 3);
    }
    
    #[test]
    fn test_musical_notes() {
        assert_eq!(MusicalNote::A4.frequency, 440.0);
        assert_eq!(MusicalNote::C4.frequency, 261.63);
        
        let midi_60 = MusicalNote::from_midi_note(60);
        assert!(midi_60.frequency > 260.0 && midi_60.frequency < 270.0);
    }
}
