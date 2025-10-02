use std::time::Duration;

/// Unique identifier for a rotation span in the LogLine timeline.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpanId(pub String);

impl SpanId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Record describing a single rotation span and its informational footprint.
#[derive(Clone, Debug)]
pub struct SpanRecord {
    pub id: SpanId,
    pub delta_entropy: f64,
    pub delta_information: f64,
    pub duration: Duration,
    pub delta_theta: f64,
    pub delta_energy: f64,
    pub gibbs_energy: f64,
}

impl SpanRecord {
    pub fn new(
        id: impl Into<String>,
        delta_entropy: f64,
        delta_information: f64,
        duration: Duration,
    ) -> Self {
        Self {
            id: SpanId(id.into()),
            delta_entropy,
            delta_information,
            duration,
            delta_theta: 0.0,
            delta_energy: 0.0,
            gibbs_energy: 0.0,
        }
    }
}

/// Aggregate trajectory of rotations executed by the folding engine.
#[derive(Clone, Debug, Default)]
pub struct Trajectory {
    spans: Vec<SpanRecord>,
    total_entropy: f64,
    total_information: f64,
    total_duration: Duration,
}

impl Trajectory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, span: SpanRecord) {
        self.total_entropy += span.delta_entropy;
        self.total_information += span.delta_information;
        self.total_duration += span.duration;
        self.spans.push(span);
    }

    pub fn spans(&self) -> &[SpanRecord] {
        &self.spans
    }

    pub fn total_entropy(&self) -> f64 {
        self.total_entropy
    }

    pub fn total_information(&self) -> f64 {
        self.total_information
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    pub fn pop_last(&mut self) -> Option<SpanRecord> {
        let span = self.spans.pop()?;
        self.total_entropy -= span.delta_entropy;
        self.total_information -= span.delta_information;
        self.total_duration = self
            .total_duration
            .checked_sub(span.duration)
            .unwrap_or_default();
        Some(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_entropy_information_and_duration() {
        let mut trajectory = Trajectory::new();
        trajectory.push(SpanRecord::new(
            "span-1",
            1.0,
            0.5,
            Duration::from_millis(2),
        ));
        trajectory.push(SpanRecord::new(
            "span-2",
            2.5,
            1.5,
            Duration::from_millis(3),
        ));

        assert_eq!(trajectory.spans().len(), 2);
        assert!((trajectory.total_entropy() - 3.5).abs() < f64::EPSILON);
        assert!((trajectory.total_information() - 2.0).abs() < f64::EPSILON);
        assert_eq!(trajectory.total_duration(), Duration::from_millis(5));

        let removed = trajectory.pop_last().expect("span removed");
        assert_eq!(removed.id.as_str(), "span-2");
        assert_eq!(trajectory.spans().len(), 1);
        assert_eq!(trajectory.total_duration(), Duration::from_millis(2));
    }
}
