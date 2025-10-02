use std::time::{Duration, Instant};

/// Monotonic clock that paces rotation execution in millisecond granularity.
pub struct RotationClock {
    start: Instant,
    base_interval: Duration,
    scale_factor: f64,
}

impl RotationClock {
    pub fn new(base_interval_ms: u64) -> Self {
        Self {
            start: Instant::now(),
            base_interval: Duration::from_millis(base_interval_ms.max(1)),
            scale_factor: 1.0,
        }
    }

    pub fn with_scale(mut self, scale_factor: f64) -> Self {
        self.scale_factor = scale_factor;
        self
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn tick_duration(&self) -> Duration {
        let scaled = self.base_interval.mul_f64(self.scale_factor);
        if scaled.is_zero() {
            self.base_interval
        } else {
            scaled
        }
    }
}
