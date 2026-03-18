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

---

## Digital Signal Processing

### Basic DSP Operations

```rust
use dasp::{signal::Signal, ring_buffer::Fixed};

fn apply_gain(signal: &[f32], gain: f32) -> Vec<f32> {
    signal.iter().map(|&sample| sample * gain).collect()
}

fn apply_low_pass_filter(signal: &[f32], cutoff: f32, sample_rate: f32) -> Vec<f32> {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);
    
    let mut filtered = Vec::with_capacity(signal.len());
    let mut prev_output = 0.0;
    
    for &input in signal {
        let output = alpha * input + (1.0 - alpha) * prev_output;
        filtered.push(output);
        prev_output = output;
    }
    
    filtered
}
```

### Fourier Transform

```rust
use rustfft::{FftPlanner, num_complex::Complex};

fn compute_fft(signal: &[f32]) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());
    
    let mut buffer: Vec<Complex<f32>> = signal
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    
    fft.process(&mut buffer);
    buffer
}

fn compute_spectrum(signal: &[f32]) -> Vec<f32> {
    let fft_result = compute_fft(signal);
    fft_result.iter().map(|c| c.norm()).collect()
}
```

---

## Audio File Handling

### Reading Audio Files

```rust
use hound::{WavReader, WavWriter, SampleFormat};

fn read_wav_file(filename: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(filename)?;
    let samples: Vec<f32> = reader.samples::<f32>()
        .map(|s| s.map(|x| x as f32))
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(samples)
}

fn write_wav_file(filename: &str, samples: &[f32], sample_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    
    let mut writer = WavWriter::create(filename, spec)?;
    for &sample in samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    
    Ok(())
}
```

### Audio Format Conversion

```rust
fn resample_audio(input: &[f32], input_rate: f32, output_rate: f32) -> Vec<f32> {
    let ratio = output_rate / input_rate;
    let output_length = (input.len() as f32 * ratio) as usize;
    let mut output = Vec::with_capacity(output_length);
    
    for i in 0..output_length {
        let input_index = (i as f32 / ratio) as usize;
        if input_index < input.len() {
            output.push(input[input_index]);
        }
    }
    
    output
}

fn apply_fade_in_out(audio: &mut [f32], fade_samples: usize) {
    let fade_length = fade_samples.min(audio.len());
    
    // Fade in
    for i in 0..fade_length {
        let gain = i as f32 / fade_length as f32;
        audio[i] *= gain;
    }
    
    // Fade out
    let start_fade = audio.len() - fade_length;
    for i in 0..fade_length {
        let gain = 1.0 - (i as f32 / fade_length as f32);
        audio[start_fade + i] *= gain;
    }
}
```

---

## Real-Time Audio

### Audio Device Setup

```rust
use cpal::{Device, Stream, StreamConfig, SampleFormat};

fn setup_audio_stream() -> Result<(Device, Stream), Box<dyn std::error::Error>> {
    let device = cpal::default_output_device()?;
    
    let config = StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };
    
    let stream = device.build_output_stream(
        &config,
        |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Process audio data here
        },
        |err| eprintln!("Error on output stream: {}", err),
    )?;
    
    Ok((device, stream))
}
```

### Audio Processing Pipeline

```rust
struct AudioProcessor {
    gain: f32,
    low_pass_cutoff: f32,
    sample_rate: f32,
}

impl AudioProcessor {
    fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            low_pass_cutoff: 1000.0,
            sample_rate,
        }
    }
    
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // Apply gain
        let with_gain: Vec<f32> = input.iter()
            .map(|&x| x * self.gain)
            .collect();
        
        // Apply low-pass filter
        let filtered = apply_low_pass_filter(&with_gain, self.low_pass_cutoff, self.sample_rate);
        
        // Copy to output
        output.copy_from_slice(&filtered[..output.len()]);
    }
    
    fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }
    
    fn set_low_pass_cutoff(&mut self, cutoff: f32) {
        self.low_pass_cutoff = cutoff;
    }
}
```

---

## Audio Analysis

### Peak Detection

```rust
fn find_peaks(signal: &[f32], threshold: f32) -> Vec<usize> {
    let mut peaks = Vec::new();
    
    for i in 1..signal.len() - 1 {
        if signal[i] > threshold && 
           signal[i] > signal[i - 1] && 
           signal[i] > signal[i + 1] {
            peaks.push(i);
        }
    }
    
    peaks
}

fn calculate_rms(signal: &[f32]) -> f32 {
    let sum_squares: f32 = signal.iter()
        .map(|&x| x * x)
        .sum();
    
    (sum_squares / signal.len() as f32).sqrt()
}

fn calculate_peak_level(signal: &[f32]) -> f32 {
    signal.iter()
        .map(|&x| x.abs())
        .fold(0.0, f32::max)
}
```

### Frequency Analysis

```rust
fn detect_dominant_frequency(signal: &[f32], sample_rate: f32) -> f32 {
    let spectrum = compute_spectrum(signal);
    
    let max_bin = spectrum.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i);
    
    if let Some(bin) = max_bin {
        bin as f32 * sample_rate / signal.len() as f32
    } else {
        0.0
    }
}

fn calculate_spectral_centroid(signal: &[f32], sample_rate: f32) -> f32 {
    let spectrum = compute_spectrum(signal);
    
    let weighted_sum: f32 = spectrum.iter()
        .enumerate()
        .map(|(i, &magnitude)| i as f32 * magnitude)
        .sum();
    
    let magnitude_sum: f32 = spectrum.iter().sum();
    
    if magnitude_sum > 0.0 {
        weighted_sum / magnitude_sum * sample_rate / signal.len() as f32
    } else {
        0.0
    }
}
```

---

## Audio Effects

### Reverb Effect

```rust
struct SimpleReverb {
    delay_buffer: Vec<f32>,
    delay_index: usize,
    feedback: f32,
    wet_level: f32,
}

impl SimpleReverb {
    fn new(delay_samples: usize, sample_rate: f32) -> Self {
        Self {
            delay_buffer: vec![0.0; delay_samples],
            delay_index: 0,
            feedback: 0.5,
            wet_level: 0.3,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.delay_buffer[self.delay_index];
        
        // Store current sample with feedback
        self.delay_buffer[self.delay_index] = input + delayed * self.feedback;
        
        // Mix dry and wet signals
        let output = input + delayed * self.wet_level;
        
        // Advance delay index
        self.delay_index = (self.delay_index + 1) % self.delay_buffer.len();
        
        output
    }
}
```

### Distortion Effect

```rust
fn apply_soft_clipping(signal: &mut [f32], threshold: f32) {
    for sample in signal.iter_mut() {
        if *sample > threshold {
            *sample = threshold + (*sample - threshold) / (1.0 + (*sample - threshold));
        } else if *sample < -threshold {
            *sample = -threshold + (*sample + threshold) / (1.0 - (*sample + threshold));
        }
    }
}

fn apply_hard_clipping(signal: &mut [f32], threshold: f32) {
    for sample in signal.iter_mut() {
        *sample = sample.max(-threshold).min(threshold);
    }
}
```

---

## Performance Considerations

### SIMD Optimization

```rust
#[cfg(target_feature = "avx2")]
fn apply_gain_simd(signal: &[f32], gain: f32) -> Vec<f32> {
    use std::arch::x86_64::*;
    
    let gain_vec = unsafe { _mm256_set1_ps(gain) };
    let mut result = Vec::with_capacity(signal.len());
    
    let chunks = signal.chunks_exact(8);
    let remainder = chunks.remainder();
    
    for chunk in chunks {
        let data = unsafe { _mm256_loadu_ps(chunk.as_ptr()) };
        let processed = unsafe { _mm256_mul_ps(data, gain_vec) };
        
        let mut output = [0.0f32; 8];
        unsafe { _mm256_storeu_ps(output.as_mut_ptr(), processed) };
        result.extend_from_slice(&output);
    }
    
    // Handle remainder
    for &sample in remainder {
        result.push(sample * gain);
    }
    
    result
}
```

### Memory Management

```rust
struct AudioBuffer {
    data: Vec<f32>,
    write_pos: usize,
    read_pos: usize,
    size: usize,
}

impl AudioBuffer {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
            write_pos: 0,
            read_pos: 0,
            size,
        }
    }
    
    fn write(&mut self, sample: f32) {
        self.data[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.size;
    }
    
    fn read(&mut self) -> f32 {
        let sample = self.data[self.read_pos];
        self.read_pos = (self.read_pos + 1) % self.size;
        sample
    }
    
    fn read_available(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        } else {
            self.size - self.read_pos + self.write_pos
        }
    }
}
```

---

## Key Takeaways

- **Audio crates** provide cross-platform audio functionality
- **DSP operations** can be implemented efficiently in Rust
- **Real-time processing** requires careful buffer management
- **FFT operations** enable frequency domain analysis
- **Audio effects** combine various processing techniques
- **Performance** can be optimized with SIMD and careful memory management

---

## Audio Processing Best Practices

| Practice | Description | Example |
|----------|-------------|---------|
| **Use appropriate crates** | Leverage existing audio libraries | `rodio`, `cpal`, `dasp` |
| **Buffer management** | Avoid allocations in real-time code | Pre-allocate buffers |
| **SIMD optimization** | Use vector instructions for performance | `std::arch` module |
| **Error handling** | Handle audio device failures gracefully | `Result` types |
| **Cross-platform** | Test on different operating systems | `#[cfg(target_os)]` |
| **Memory safety** | Avoid buffer overflows | Bounds checking |
| **Performance profiling** | Measure audio processing performance | `criterion` benchmarking |
