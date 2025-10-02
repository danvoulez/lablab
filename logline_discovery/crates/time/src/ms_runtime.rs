use std::collections::VecDeque;
use std::time::Duration;

use crate::rotation_clock::RotationClock;

/// Schedules rotation tasks according to the rotation clock pacing.
pub struct MsRuntimeExecutor<T> {
    clock: RotationClock,
    queue: VecDeque<T>,
}

impl<T> MsRuntimeExecutor<T> {
    pub fn new(clock: RotationClock) -> Self {
        Self {
            clock,
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn tick_interval(&self) -> Duration {
        self.clock.tick_duration()
    }
}
