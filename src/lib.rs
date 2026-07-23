use std::f32::consts::TAU;
use std::fs;
use std::path::Path;

pub mod click;
pub mod clock;
pub mod playback;

pub const SAMPLE_RATE: u32 = 44_100;
pub const FREQUENCY_HZ: f32 = 440.0;
pub const DURATION_SECONDS: f32 = 1.0;

pub fn generate_sine_wave(sample_rate: u32, frequency_hz: f32, duration_seconds: f32) -> Vec<f32> {
    let sample_count = (sample_rate as f32 * duration_seconds) as usize;

    (0..sample_count)
        .map(|sample_index| {
            let time_seconds = sample_index as f32 / sample_rate as f32;
            (TAU * frequency_hz * time_seconds).sin()
        })
        .collect()
}

pub fn write_float_wav<P: AsRef<Path>>(
    path: P,
    samples: &[f32],
    sample_rate: u32,
) -> hound::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    for sample in samples {
        writer.write_sample(*sample)?;
    }
    writer.finalize()
}

pub fn write_default_sine_wav<P: AsRef<Path>>(path: P) -> hound::Result<()> {
    let samples = generate_sine_wave(SAMPLE_RATE, FREQUENCY_HZ, DURATION_SECONDS);
    write_float_wav(path, &samples, SAMPLE_RATE)
}

pub fn write_default_click_wav<P: AsRef<Path>>(path: P) -> hound::Result<()> {
    let samples = click::generate_default_click_track(SAMPLE_RATE);
    write_float_wav(path, &samples, SAMPLE_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_sine_has_expected_sample_count() {
        let samples = generate_sine_wave(SAMPLE_RATE, FREQUENCY_HZ, DURATION_SECONDS);

        assert_eq!(samples.len(), 44_100);
    }

    #[test]
    fn generated_sine_amplitude_stays_in_unit_range() {
        let samples = generate_sine_wave(SAMPLE_RATE, FREQUENCY_HZ, DURATION_SECONDS);

        assert!(samples.iter().all(|sample| (-1.0..=1.0).contains(sample)));
    }

    #[test]
    fn generated_sine_first_sample_is_approximately_zero() {
        let samples = generate_sine_wave(SAMPLE_RATE, FREQUENCY_HZ, DURATION_SECONDS);

        assert!(samples[0].abs() < f32::EPSILON);
    }
}
