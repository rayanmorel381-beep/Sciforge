//! Metrics collection: named counters and timers.
//!
//! [`Metrics`] is thread-safe (`Mutex`) and allows instrumenting
//! any operation via [`time_operation`](Metrics::time_operation).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// Thread-safe counter and timing collector.
pub struct Metrics {
    counters: Mutex<HashMap<String, u64>>,
    timings: Mutex<HashMap<String, Vec<f64>>>,
}

impl Metrics {
    /// Creates an empty collector.
    pub fn new() -> Self {
        Self {
            counters: Mutex::new(HashMap::new()),
            timings: Mutex::new(HashMap::new()),
        }
    }

    /// Increments counter `name` by 1.
    pub fn increment(&self, name: &str) {
        if let Ok(mut c) = self.counters.lock() {
            *c.entry(name.to_string()).or_insert(0) += 1;
        }
    }

    /// Increments counter `name` by `amount`.
    pub fn increment_by(&self, name: &str, amount: u64) {
        if let Ok(mut c) = self.counters.lock() {
            *c.entry(name.to_string()).or_insert(0) += amount;
        }
    }

    /// Returns the current value of counter `name` (0 if nonexistent).
    pub fn counter(&self, name: &str) -> u64 {
        self.counters
            .lock()
            .ok()
            .and_then(|c| c.get(name).copied())
            .unwrap_or(0)
    }

    /// Records a `duration_ms` timing for timer `name`.
    pub fn record_timing(&self, name: &str, duration_ms: f64) {
        if let Ok(mut t) = self.timings.lock() {
            t.entry(name.to_string()).or_default().push(duration_ms);
        }
    }

    /// Returns `(mean, min, max, count)` for timer `name`.
    pub fn timing_stats(&self, name: &str) -> Option<(f64, f64, f64, usize)> {
        self.timings.lock().ok().and_then(|t| {
            t.get(name).map(|vals| {
                let count = vals.len();
                let sum: f64 = vals.iter().sum();
                let mean = sum / count.max(1) as f64;
                let min = vals.iter().copied().fold(f64::INFINITY, f64::min);
                let max = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                (mean, min, max, count)
            })
        })
    }

    /// Executes `op` and records its duration under timer `name`.
    pub fn time_operation<F, R>(&self, name: &str, op: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = op();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        self.record_timing(name, elapsed);
        result
    }

    /// Resets all counters and timers.
    pub fn reset(&self) {
        if let Ok(mut c) = self.counters.lock() {
            c.clear();
        }
        if let Ok(mut t) = self.timings.lock() {
            t.clear();
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
