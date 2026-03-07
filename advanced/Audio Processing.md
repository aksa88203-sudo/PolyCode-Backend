# Audio Processing in Rust

## Overview

Rust's performance and memory safety make it excellent for audio processing applications. This guide covers digital signal processing, audio file handling, real-time audio, and building audio applications in Rust.

---

## Audio Processing Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `rodio` | Audio playback | Cross-platform audio output |
| `cpal` | Audio capture | Cross-platform audio input |
| `hound` | WAV file handling | Read/write WAV files |
| `symphonia` | Audio format support | Multiple audio formats |
| `rustfft` | FFT operations | Fast Fourier Transform |
| `dasp` | Digital signal processing | DSP utilities |
| `portaudio` | PortAudio bindings | Low-level audio I/O |
| `jack` | JACK audio server | Professional audio |

---

## Basic Audio Concepts

### Audio Fundamentals

```rust
use std::f32::consts::PI;

// Audio sample types
type Sample = f32; // 32-bit float samples
type StereoSample = (Sample, Sample); // Left, Right channels
type MonoFrame = Sample;
type StereoFrame = [Sample; 2];

// Audio parameters
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,      // Samples per second (44100, 48000, etc.)
    pub channels: u16,          // Number of audio channels (1=mono, 2=stereo)
    pub buffer_size: usize,     // Size of audio buffer
    pub bit_depth: u16,        // Bits per sample (16, 24, 32)
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

impl AudioConfig {
    pub fn duration_to_samples(&self, duration: std::time::Duration) -> usize {
        let seconds = duration.as_secs_f64();
        (seconds * self.sample_rate as f64) as usize
    }
    
    pub fn samples_to_duration(&self, samples: usize) -> std::time::Duration {
        let seconds = samples as f64 / self.sample_rate as f64;
        std::time::Duration::from_secs_f64(seconds)
    }
    
    pub fn bytes_per_second(&self) -> usize {
        (self.sample_rate as usize) * 
        (self.channels as usize) * 
        (self.bit_depth as usize / 8)
    }
}
```

### Wave Generation

```rust
pub struct WaveGenerator {
    config: AudioConfig,
    phase: f32,
    frequency: f32,
    amplitude: f32,
    wave_type: WaveType,
}

#[derive(Debug, Clone, Copy)]
pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise,
}

impl WaveGenerator {
    pub fn new(config: AudioConfig) -> Self {
        WaveGenerator {
            config,
            phase: 0.0,
            frequency: 440.0, // A4 note
            amplitude: 0.5,
            wave_type: WaveType::Sine,
        }
    }
    
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude.clamp(-1.0, 1.0);
    }
    
    pub fn set_wave_type(&mut self, wave_type: WaveType) {
        self.wave_type = wave_type;
    }
    
    pub fn generate_samples(&mut self, buffer: &mut [Sample]) {
        for sample in buffer.iter_mut() {
            *sample = self.generate_sample();
            self.advance_phase();
        }
    }
    
    fn generate_sample(&self) -> Sample {
        let phase_normalized = self.phase / (2.0 * PI);
        
        match self.wave_type {
            WaveType::Sine => self.amplitude * (self.phase).sin(),
            WaveType::Square => {
                if self.phase < PI {
                    self.amplitude
                } else {
                    -self.amplitude
                }
            },
            WaveType::Sawtooth => {
                self.amplitude * (2.0 * phase_normalized - 1.0)
            },
            WaveType::Triangle => {
                let abs_phase = (phase_normalized - 0.5).abs() * 4.0;
                if abs_phase < 1.0 {
                    self.amplitude * (2.0 * abs_phase - 1.0)
                } else {
                    self.amplitude * (3.0 - 2.0 * abs_phase)
                }
            },
            WaveType::Noise => {
                self.amplitude * (rand::random::<f32>() * 2.0 - 1.0)
            },
        }
    }
    
    fn advance_phase(&mut self) {
        let phase_increment = 2.0 * PI * self.frequency / self.config.sample_rate as f32;
        self.phase += phase_increment;
        
        // Wrap phase to [0, 2π]
        while self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
    }
    
    pub fn generate_tone(&mut self, duration: std::time::Duration) -> Vec<Sample> {
        let num_samples = self.config.duration_to_samples(duration);
        let mut buffer = vec![0.0; num_samples];
        self.generate_samples(&mut buffer);
        buffer
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
    pub const D4: MusicalNote = MusicalNote { frequency: 293.66, name: "D4" };
    pub const E4: MusicalNote = MusicalNote { frequency: 329.63, name: "E4" };
    pub const F4: MusicalNote = MusicalNote { frequency: 349.23, name: "F4" };
    pub const G4: MusicalNote = MusicalNote { frequency: 392.00, name: "G4" };
    pub const A5: MusicalNote = MusicalNote { frequency: 880.00, name: "A5" };
    pub const B4: MusicalNote = MusicalNote { frequency: 493.88, name: "B4" };
    
    pub fn from_midi_note(midi_note: u8) -> MusicalNote {
        // A4 (MIDI note 69) = 440 Hz
        let a4_freq = 440.0;
        let a4_midi = 69;
        
        let freq = a4_freq * 2.0f32.powf((midi_note as f32 - a4_midi as f32) / 12.0);
        
        MusicalNote {
            frequency: freq,
            name: "MIDI",
        }
    }
}
```

---

## Digital Signal Processing

### Basic DSP Operations

```rust
pub struct DspProcessor {
    config: AudioConfig,
    delay_line: Vec<Sample>,
    delay_index: usize,
    filter_coeffs: Vec<Sample>,
    filter_history: Vec<Sample>,
}

impl DspProcessor {
    pub fn new(config: AudioConfig) -> Self {
        DspProcessor {
            config,
            delay_line: vec![0.0; 44100], // 1 second delay
            delay_index: 0,
            filter_coeffs: Vec::new(),
            filter_history: Vec::new(),
        }
    }
    
    // Simple delay effect
    pub fn apply_delay(&mut self, input: &[Sample], output: &mut [Sample], delay_ms: f32) {
        let delay_samples = (delay_ms / 1000.0 * self.config.sample_rate as f32) as usize;
        
        for (i, &sample) in input.iter().enumerate() {
            let delayed_sample = self.delay_line[(self.delay_index + self.delay_line.len() - delay_samples) % self.delay_line.len()];
            
            // Mix original and delayed signal
            output[i] = sample + delayed_sample * 0.5;
            
            // Update delay line
            self.delay_line[self.delay_index] = sample;
            self.delay_index = (self.delay_index + 1) % self.delay_line.len();
        }
    }
    
    // Simple low-pass filter
    pub fn apply_lowpass_filter(&mut self, input: &[Sample], output: &mut [Sample], cutoff_freq: f32) {
        let dt = 1.0 / self.config.sample_rate as f32;
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let alpha = dt / (rc + dt);
        
        if self.filter_history.len() < 1 {
            self.filter_history.resize(1, 0.0);
        }
        
        for (i, &sample) in input.iter().enumerate() {
            let filtered = alpha * sample + (1.0 - alpha) * self.filter_history[0];
            output[i] = filtered;
            self.filter_history[0] = filtered;
        }
    }
    
    // Simple high-pass filter
    pub fn apply_highpass_filter(&mut self, input: &[Sample], output: &mut [Sample], cutoff_freq: f32) {
        let dt = 1.0 / self.config.sample_rate as f32;
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let alpha = rc / (rc + dt);
        
        if self.filter_history.len() < 2 {
            self.filter_history.resize(2, 0.0);
        }
        
        for (i, &sample) in input.iter().enumerate() {
            let filtered = alpha * (self.filter_history[1] + sample - self.filter_history[0]);
            output[i] = filtered;
            
            self.filter_history[1] = self.filter_history[0];
            self.filter_history[0] = filtered;
        }
    }
    
    // Simple compressor
    pub fn apply_compressor(&mut self, input: &[Sample], output: &mut [Sample], 
                           threshold: f32, ratio: f32, attack: f32, release: f32) {
        let attack_coeff = (-1.0 / (attack * self.config.sample_rate as f32)).exp();
        let release_coeff = (-1.0 / (release * self.config.sample_rate as f32)).exp();
        
        let mut envelope = 0.0;
        
        for (i, &sample) in input.iter().enumerate() {
            let input_level = sample.abs();
            
            // Envelope follower
            if input_level > envelope {
                envelope = attack_coeff * envelope + (1.0 - attack_coeff) * input_level;
            } else {
                envelope = release_coeff * envelope + (1.0 - release_coeff) * input_level;
            }
            
            // Apply compression
            let gain_reduction = if envelope > threshold {
                1.0 / (1.0 + (ratio - 1.0) * ((envelope - threshold) / threshold))
            } else {
                1.0
            };
            
            output[i] = sample * gain_reduction;
        }
    }
    
    // Simple reverb using multiple delays
    pub fn apply_reverb(&mut self, input: &[Sample], output: &mut [Sample], 
                      room_size: f32, damping: f32) {
        let num_delays = 4;
        let delay_times = [0.03, 0.05, 0.07, 0.09]; // seconds
        let gains = [0.7, 0.5, 0.3, 0.2];
        
        for (i, &sample) in input.iter().enumerate() {
            let mut reverb_signal = 0.0;
            
            for j in 0..num_delays {
                let delay_samples = (delay_times[j] * self.config.sample_rate as f32) as usize;
                let delayed_sample = self.delay_line[(self.delay_index + self.delay_line.len() - delay_samples) % self.delay_line.len()];
                reverb_signal += delayed_sample * gains[j];
            }
            
            output[i] = sample + reverb_signal * room_size * damping;
        }
        
        // Update delay line
        for &sample in input.iter() {
            self.delay_line[self.delay_index] = sample;
            self.delay_index = (self.delay_index + 1) % self.delay_line.len();
        }
    }
}
```

### FFT Analysis

```rust
use rustfft::{FftPlanner, num_complex::Complex};

pub struct SpectrumAnalyzer {
    config: AudioConfig,
    fft_planner: FftPlanner<f32>,
    fft_size: usize,
    window: Vec<Sample>,
    spectrum: Vec<Complex<Sample>>,
}

impl SpectrumAnalyzer {
    pub fn new(config: AudioConfig) -> Self {
        let fft_size = config.buffer_size.next_power_of_two();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(fft_size);
        
        SpectrumAnalyzer {
            config,
            fft_planner: planner,
            fft_size,
            window: Self::create_hann_window(fft_size),
            spectrum: vec![Complex::new(0.0, 0.0); fft_size],
        }
    }
    
    fn create_hann_window(size: usize) -> Vec<Sample> {
        let mut window = Vec::with_capacity(size);
        
        for i in 0..size {
            let sample = 0.5 * (1.0 - (2.0 * PI * i as f32 / (size - 1) as f32).cos());
            window.push(sample);
        }
        
        window
    }
    
    pub fn analyze(&mut self, samples: &[Sample]) -> Vec<f32> {
        // Apply window function
        let mut windowed_samples = vec![0.0; self.fft_size];
        let samples_to_process = samples.len().min(self.fft_size);
        
        for i in 0..samples_to_process {
            windowed_samples[i] = samples[i] * self.window[i];
        }
        
        // Convert to complex
        for i in 0..self.fft_size {
            self.spectrum[i] = Complex::new(windowed_samples[i], 0.0);
        }
        
        // Perform FFT
        let fft = self.fft_planner.plan_fft(self.fft_size);
        fft.process(&mut self.spectrum);
        
        // Convert to magnitude spectrum
        let mut magnitudes = Vec::with_capacity(self.fft_size / 2);
        
        for i in 0..self.fft_size / 2 {
            let magnitude = self.spectrum[i].norm();
            magnitudes.push(magnitude);
        }
        
        magnitudes
    }
    
    pub fn get_frequency_bins(&self) -> Vec<f32> {
        let nyquist = self.config.sample_rate as f32 / 2.0;
        let bin_width = nyquist / (self.fft_size / 2) as f32;
        
        (0..self.fft_size / 2)
            .map(|i| i as f32 * bin_width)
            .collect()
    }
    
    pub fn find_peaks(&self, magnitudes: &[f32], threshold: f32) -> Vec<(usize, f32)> {
        let mut peaks = Vec::new();
        
        for i in 1..magnitudes.len() - 1 {
            if magnitudes[i] > magnitudes[i - 1] && 
               magnitudes[i] > magnitudes[i + 1] && 
               magnitudes[i] > threshold {
                peaks.push((i, magnitudes[i]));
            }
        }
        
        peaks
    }
    
    pub fn get_dominant_frequency(&self, magnitudes: &[f32]) -> f32 {
        let max_index = magnitudes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let frequency_bins = self.get_frequency_bins();
        frequency_bins[max_index]
    }
}
```

---

## Audio File Handling

### WAV File Operations

```rust
use hound::{WavReader, WavWriter, WavSpec};

pub struct WavHandler;

impl WavHandler {
    pub fn read_wav_file(path: &str) -> Result<(Vec<f32>, WavSpec), Box<dyn std::error::Error>> {
        let mut reader = WavReader::open(path)?;
        let spec = reader.spec();
        
        let samples: Vec<f32> = if spec.bits_per_sample == 16 {
            reader.into_samples::<i16>()
                .map(|s| s.map(|sample| sample as f32 / i16::MAX as f32))
                .filter_map(Result::ok)
                .collect()
        } else if spec.bits_per_sample == 24 {
            reader.into_samples::<i32>()
                .map(|s| s.map(|sample| sample as f32 / i32::MAX as f32))
                .filter_map(Result::ok)
                .collect()
        } else {
            reader.into_samples::<f32>()
                .filter_map(Result::ok)
                .collect()
        };
        
        Ok((samples, spec))
    }
    
    pub fn write_wav_file(path: &str, samples: &[f32], spec: WavSpec) -> Result<(), Box<dyn std::error::Error>> {
        let writer = WavWriter::create(path, spec)?;
        
        for &sample in samples {
            if spec.bits_per_sample == 16 {
                writer.write_sample((sample * i16::MAX as f32) as i16)?;
            } else {
                writer.write_sample(sample)?;
            }
        }
        
        writer.finalize()?;
        Ok(())
    }
    
    pub fn create_test_wav() -> Result<(), Box<dyn std::error::Error>> {
        let spec = WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        
        let duration = std::time::Duration::from_secs(5);
        let num_samples = (duration.as_secs_f64() * spec.sample_rate as f64) as usize;
        let mut samples = Vec::with_capacity(num_samples);
        
        // Generate a simple sine wave
        for i in 0..num_samples {
            let t = i as f32 / spec.sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
            
            // Stereo samples
            samples.push(sample);
            samples.push(sample);
        }
        
        Self::write_wav_file("test_sine.wav", &samples, spec)?;
        println!("Created test_sine.wav with {} samples", samples.len() / 2);
        
        Ok(())
    }
}
```

### Audio Format Conversion

```rust
pub struct AudioConverter;

impl AudioConverter {
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
            
            // Linear interpolation
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
    
    pub fn apply_gain(samples: &mut [f32], gain_db: f32) {
        let gain_linear = 10.0f32.powf(gain_db / 20.0);
        
        for sample in samples.iter_mut() {
            *sample *= gain_linear;
        }
    }
    
    pub fn apply_fade_in(samples: &mut [f32], duration_ms: f32, sample_rate: u32) {
        let fade_samples = (duration_ms / 1000.0 * sample_rate as f32) as usize;
        let fade_samples = fade_samples.min(samples.len());
        
        for (i, sample) in samples.iter_mut().enumerate().take(fade_samples) {
            let gain = i as f32 / fade_samples as f32;
            *sample *= gain;
        }
    }
    
    pub fn apply_fade_out(samples: &mut [f32], duration_ms: f32, sample_rate: u32) {
        let fade_samples = (duration_ms / 1000.0 * sample_rate as f32) as usize;
        let fade_samples = fade_samples.min(samples.len());
        let start = samples.len() - fade_samples;
        
        for (i, sample) in samples.iter_mut().enumerate().skip(start) {
            let progress = (i - start) as f32 / fade_samples as f32;
            let gain = 1.0 - progress;
            *sample *= gain;
        }
    }
}
```

---

## Real-Time Audio

### Audio Stream Processing

```rust
use cpal::{Device, Stream, StreamConfig, SampleFormat, traits::{DeviceTrait, HostTrait, StreamTrait}};

pub struct AudioProcessor {
    config: AudioConfig,
    wave_generator: WaveGenerator,
    dsp_processor: DspProcessor,
    spectrum_analyzer: SpectrumAnalyzer,
}

impl AudioProcessor {
    pub fn new(config: AudioConfig) -> Self {
        AudioProcessor {
            wave_generator: WaveGenerator::new(config.clone()),
            dsp_processor: DspProcessor::new(config.clone()),
            spectrum_analyzer: SpectrumAnalyzer::new(config),
        }
    }
    
    pub fn process_input_callback(&mut self, input: &[f32], output: &mut [f32]) {
        // Apply DSP effects to input
        let mut processed = vec![0.0; input.len()];
        self.dsp_processor.apply_delay(input, &mut processed, 0.1); // 100ms delay
        self.dsp_processor.apply_lowpass_filter(&processed, output, 1000.0); // 1kHz cutoff
    }
    
    pub fn process_output_callback(&mut self, output: &mut [f32]) {
        // Generate audio
        self.wave_generator.generate_samples(output);
        
        // Apply effects
        let mut effected = vec![0.0; output.len()];
        self.dsp_processor.apply_compressor(output, &mut effected, -12.0, 4.0, 0.001, 0.1);
        output.copy_from_slice(&effected);
    }
    
    pub fn analyze_audio(&mut self, samples: &[f32]) -> Vec<f32> {
        self.spectrum_analyzer.analyze(samples)
    }
}

pub struct AudioStream {
    device: Device,
    config: StreamConfig,
    processor: AudioProcessor,
}

impl AudioStream {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or("No output device available")?;
        
        let config = StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(44100),
            buffer_size: cpal::BufferSize::Fixed(512),
        };
        
        let audio_config = AudioConfig {
            sample_rate: config.sample_rate.0,
            channels: config.channels,
            buffer_size: 512,
            bit_depth: 32,
        };
        
        let processor = AudioProcessor::new(audio_config);
        
        Ok(AudioStream {
            device,
            config,
            processor,
        })
    }
    
    pub fn start_input_stream(&mut self) -> Result<Stream<f32>, Box<dyn std::error::Error>> {
        let stream = self.device.build_input_stream(
            &self.config,
            |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Process input data
                // This would be called in real-time
            },
            |err| eprintln!("Input stream error: {}", err),
        )?;
        
        stream.play()?;
        Ok(stream)
    }
    
    pub fn start_output_stream(&mut self) -> Result<Stream<f32>, Box<dyn std::error::Error>> {
        let stream = self.device.build_output_stream(
            &self.config,
            |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                self.processor.process_output_callback(data);
            },
            |err| eprintln!("Output stream error: {}", err),
        )?;
        
        stream.play()?;
        Ok(stream)
    }
}
```

---

## Audio Effects

### Advanced Effects

```rust
pub struct AudioEffects {
    config: AudioConfig,
    chorus_delay_line: Vec<f32>,
    chorus_index: usize,
    flanger_delay_line: Vec<f32>,
    flanger_index: usize,
    vibrato_phase: f32,
}

impl AudioEffects {
    pub fn new(config: AudioConfig) -> Self {
        AudioEffects {
            config,
            chorus_delay_line: vec![0.0; config.sample_rate as usize / 10], // 100ms max delay
            chorus_index: 0,
            flanger_delay_line: vec![0.0; config.sample_rate as usize / 10], // 100ms max delay
            flanger_index: 0,
            vibrato_phase: 0.0,
        }
    }
    
    // Chorus effect
    pub fn apply_chorus(&mut self, input: &[f32], output: &mut [f32], 
                     delay_ms: f32, depth: f32, rate: f32) {
        let delay_samples = (delay_ms / 1000.0 * self.config.sample_rate as f32) as usize;
        let lfo_phase_increment = 2.0 * std::f32::consts::PI * rate / self.config.sample_rate as f32;
        let mut lfo_phase = 0.0;
        
        for (i, &sample) in input.iter().enumerate() {
            let lfo = (lfo_phase).sin();
            let modulated_delay = delay_samples + ((depth * lfo) / 1000.0 * self.config.sample_rate as f32) as usize;
            
            let delayed_sample = self.chorus_delay_line[
                (self.chorus_index + self.chorus_delay_line.len() - modulated_delay) % self.chorus_delay_line.len()
            ];
            
            output[i] = sample + delayed_sample * 0.5;
            
            // Update delay line
            self.chorus_delay_line[self.chorus_index] = sample;
            self.chorus_index = (self.chorus_index + 1) % self.chorus_delay_line.len();
            
            lfo_phase += lfo_phase_increment;
            if lfo_phase >= 2.0 * std::f32::consts::PI {
                lfo_phase -= 2.0 * std::f32::consts::PI;
            }
        }
    }
    
    // Flanger effect
    pub fn apply_flanger(&mut self, input: &[f32], output: &mut [f32], 
                     delay_ms: f32, depth: f32, rate: f32) {
        let delay_samples = (delay_ms / 1000.0 * self.config.sample_rate as f32) as usize;
        let lfo_phase_increment = 2.0 * std::f32::consts::PI * rate / self.config.sample_rate as f32;
        let mut lfo_phase = 0.0;
        
        for (i, &sample) in input.iter().enumerate() {
            let lfo = (lfo_phase).sin();
            let modulated_delay = delay_samples + ((depth * lfo) / 1000.0 * self.config.sample_rate as f32) as usize;
            
            let delayed_sample = self.flanger_delay_line[
                (self.flanger_index + self.flanger_delay_line.len() - modulated_delay) % self.flanger_delay_line.len()
            ];
            
            output[i] = sample + delayed_sample * 0.7;
            
            // Update delay line
            self.flanger_delay_line[self.flanger_index] = sample;
            self.flanger_index = (self.flanger_index + 1) % self.flanger_delay_line.len();
            
            lfo_phase += lfo_phase_increment;
            if lfo_phase >= 2.0 * std::f32::consts::PI {
                lfo_phase -= 2.0 * std::f32::consts::PI;
            }
        }
    }
    
    // Vibrato effect
    pub fn apply_vibrato(&mut self, input: &[f32], output: &mut [f32], 
                      depth: f32, rate: f32) {
        let lfo_phase_increment = 2.0 * std::f32::consts::PI * rate / self.config.sample_rate as f32;
        
        for (i, &sample) in input.iter().enumerate() {
            let lfo = (self.vibrato_phase).sin();
            let vibrato_amount = depth * lfo;
            
            // Simple pitch modulation using sample interpolation
            output[i] = sample * (1.0 + vibrato_amount);
            
            self.vibrato_phase += lfo_phase_increment;
            if self.vibrato_phase >= 2.0 * std::f32::consts::PI {
                self.vibrato_phase -= 2.0 * std::f32::consts::PI;
            }
        }
    }
    
    // Distortion effect
    pub fn apply_distortion(input: &[f32], output: &mut [f32], drive: f32, mix: f32) {
        for (i, &sample) in input.iter().enumerate() {
            let driven = sample * drive;
            let distorted = if driven > 1.0 {
                1.0
            } else if driven < -1.0 {
                -1.0
            } else {
                driven
            };
            
            output[i] = sample * (1.0 - mix) + distorted * mix;
        }
    }
    
    // Bit crusher effect
    pub fn apply_bit_crusher(input: &[f32], output: &mut [f32], bit_depth: u8, sample_rate_reduction: u32) {
        let quantization_levels = (1 << bit_depth) as f32;
        let sample_skip = sample_rate_reduction as usize;
        
        for (i, &sample) in input.iter().enumerate() {
            if i % sample_skip == 0 {
                let quantized = (sample * quantization_levels).round() / quantization_levels;
                output[i] = quantized;
            } else {
                output[i] = output[i - 1]; // Hold previous sample
            }
        }
    }
}
```

---

## Key Takeaways

- **Audio processing** requires understanding of digital signal processing concepts
- **Sample rate** and **bit depth** determine audio quality
- **FFT** enables frequency domain analysis and processing
- **Real-time audio** requires efficient, low-latency processing
- **Effects** are implemented using DSP algorithms
- **Cross-platform** audio libraries handle OS differences
- **Performance** is critical for real-time applications

---

## Audio Processing Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Buffer management** | Efficient audio buffer handling | Use circular buffers |
| **Avoid allocations** | Prevent audio dropouts | Pre-allocate buffers |
| **Sample rate consistency** | Handle different sample rates | Resample as needed |
| **Clipping prevention** | Avoid audio distortion | Normalize and limit output |
| **Thread safety** | Handle concurrent access | Use proper synchronization |
| **Error handling** | Graceful audio error handling | Check audio device status |
| **Performance optimization** | Optimize DSP algorithms | Use SIMD when possible |
