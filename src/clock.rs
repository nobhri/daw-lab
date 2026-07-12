#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    sample_rate: u32,
    position_samples: u64,
}

impl Clock {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            position_samples: 0,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn position_samples(&self) -> u64 {
        self.position_samples
    }

    pub fn position_seconds(&self) -> f64 {
        self.position_samples as f64 / f64::from(self.sample_rate)
    }

    pub fn advance(&mut self, samples: u64) {
        self.position_samples += samples;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_starts_at_sample_zero() {
        let clock = Clock::new(44_100);

        assert_eq!(clock.sample_rate(), 44_100);
        assert_eq!(clock.position_samples(), 0);
        assert_eq!(clock.position_seconds(), 0.0);
    }

    #[test]
    fn clock_advances_by_one_sample() {
        let mut clock = Clock::new(44_100);

        clock.advance(1);

        assert_eq!(clock.position_samples(), 1);
    }

    #[test]
    fn clock_advances_by_multiple_samples() {
        let mut clock = Clock::new(44_100);

        clock.advance(512);
        clock.advance(256);

        assert_eq!(clock.position_samples(), 768);
    }

    #[test]
    fn clock_converts_sample_position_to_seconds() {
        let mut clock = Clock::new(44_100);

        clock.advance(44_100);

        assert_eq!(clock.position_seconds(), 1.0);
    }

    #[test]
    fn clock_converts_fractional_seconds() {
        let mut clock = Clock::new(48_000);

        clock.advance(12_000);

        assert_eq!(clock.position_seconds(), 0.25);
    }
}
