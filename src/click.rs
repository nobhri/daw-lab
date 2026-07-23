use std::f32::consts::TAU;

pub const DEFAULT_BPM: f32 = 120.0;
pub const DEFAULT_DURATION_SECONDS: u32 = 4;
pub const DEFAULT_FREQUENCY_HZ: f32 = 1_000.0;
pub const DEFAULT_CLICK_DURATION_SECONDS: f32 = 0.010;

pub fn samples_per_beat(sample_rate: u32, bpm: f32) -> u64 {
    assert!(sample_rate > 0, "sample rate must be greater than zero");
    assert!(bpm.is_finite() && bpm > 0.0, "BPM must be positive");

    (f64::from(sample_rate) * 60.0 / f64::from(bpm)).round() as u64
}

pub fn click_sample(
    sample_position: u64,
    sample_rate: u32,
    bpm: f32,
    frequency_hz: f32,
    click_duration_seconds: f32,
) -> f32 {
    let beat_interval = samples_per_beat(sample_rate, bpm);
    let click_length = (sample_rate as f32 * click_duration_seconds).round() as u64;
    let position_in_beat = sample_position % beat_interval;

    if click_length == 0 || position_in_beat >= click_length {
        return 0.0;
    }

    let time_seconds = position_in_beat as f32 / sample_rate as f32;
    let envelope = 1.0 - position_in_beat as f32 / click_length as f32;

    (TAU * frequency_hz * time_seconds).cos() * envelope
}

pub fn generate_click_track(
    sample_rate: u32,
    bpm: f32,
    duration_seconds: u32,
    frequency_hz: f32,
    click_duration_seconds: f32,
) -> Vec<f32> {
    let sample_count = u64::from(sample_rate) * u64::from(duration_seconds);

    (0..sample_count)
        .map(|position| {
            click_sample(
                position,
                sample_rate,
                bpm,
                frequency_hz,
                click_duration_seconds,
            )
        })
        .collect()
}

pub fn generate_default_click_track(sample_rate: u32) -> Vec<f32> {
    generate_click_track(
        sample_rate,
        DEFAULT_BPM,
        DEFAULT_DURATION_SECONDS,
        DEFAULT_FREQUENCY_HZ,
        DEFAULT_CLICK_DURATION_SECONDS,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: u32 = 44_100;

    #[test]
    fn beat_interval_at_120_bpm_is_22_050_frames() {
        assert_eq!(samples_per_beat(SAMPLE_RATE, DEFAULT_BPM), 22_050);
    }

    #[test]
    fn click_starts_at_sample_zero() {
        let sample = click_sample(
            0,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );

        assert_eq!(sample, 1.0);
    }

    #[test]
    fn another_click_starts_at_the_next_beat() {
        let first_click = click_sample(
            0,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );
        let second_click = click_sample(
            22_050,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );

        assert_eq!(first_click, second_click);
    }

    #[test]
    fn position_between_clicks_is_silent() {
        let sample = click_sample(
            1_000,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );

        assert_eq!(sample, 0.0);
    }

    #[test]
    fn fade_out_reduces_click_amplitude() {
        let early = click_sample(
            0,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );
        let late = click_sample(
            430,
            SAMPLE_RATE,
            DEFAULT_BPM,
            DEFAULT_FREQUENCY_HZ,
            DEFAULT_CLICK_DURATION_SECONDS,
        );

        assert!(late.abs() < early.abs());
    }

    #[test]
    fn generated_samples_stay_in_unit_range() {
        let samples = generate_default_click_track(SAMPLE_RATE);

        assert!(samples.iter().all(|sample| (-1.0..=1.0).contains(sample)));
    }

    #[test]
    fn four_second_render_has_expected_sample_count() {
        let samples = generate_default_click_track(SAMPLE_RATE);

        assert_eq!(samples.len(), 176_400);
    }
}
