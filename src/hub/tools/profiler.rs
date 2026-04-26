//! Experiment profiling: individual and batch timing,
//! statistics (min, max, median, stddev), and CSV / Markdown export.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::experiment::{DomainType, Experiment, ParameterValue};
use crate::hub::engine::experience::runner::{ExperimentRunner, RunOutput};
use std::time::Instant;

/// Profiling result for a single experiment.
#[derive(Debug, Clone)]
pub struct ProfileEntry {
    /// Scientific domain being profiled.
    pub domain: String,
    /// Name of the profiled function.
    pub function_name: String,
    /// Number of iterations performed.
    pub iterations: u64,
    /// Total time in nanoseconds.
    pub total_ns: u128,
    /// Minimum time in nanoseconds.
    pub min_ns: u128,
    /// Maximum time in nanoseconds.
    pub max_ns: u128,
    /// Mean time in nanoseconds.
    pub mean_ns: f64,
    /// Standard deviation in nanoseconds.
    pub stddev_ns: f64,
    /// Median time in nanoseconds.
    pub median_ns: u128,
}

impl ProfileEntry {
    /// Throughput in calls per second, derived from the mean.
    pub fn throughput_per_sec(&self) -> f64 {
        if self.mean_ns == 0.0 {
            return 0.0;
        }
        1e9 / self.mean_ns
    }

    /// Serializes the entry as a CSV row.
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{:.1},{:.1},{}",
            self.domain,
            self.function_name,
            self.iterations,
            self.total_ns,
            self.min_ns,
            self.max_ns,
            self.mean_ns,
            self.stddev_ns,
            self.median_ns,
        )
    }
}

/// CSV header for profiling columns.
pub const PROFILE_CSV_HEADER: &str =
    "domain,function,iterations,total_ns,min_ns,max_ns,mean_ns,stddev_ns,median_ns";

/// Profiles an experiment over `iterations` runs and returns statistics.
pub fn profile_experiment(experiment: &Experiment, iterations: u64) -> HubResult<ProfileEntry> {
    if iterations == 0 {
        return Err(HubError::InvalidInput("iterations must be > 0".into()));
    }
    let runner = ExperimentRunner::new();
    let _ = runner.run(experiment)?;
    let mut timings = Vec::with_capacity(iterations as usize);
    let mut total: u128 = 0;

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = runner.run(experiment);
        let elapsed = start.elapsed().as_nanos();
        timings.push(elapsed);
        total += elapsed;
    }

    timings.sort_unstable();
    let n = timings.len();
    let min_ns = timings[0];
    let max_ns = timings[n - 1];
    let median_ns = timings[n / 2];
    let mean_ns = total as f64 / n as f64;
    let variance = timings
        .iter()
        .map(|&t| {
            let d = t as f64 - mean_ns;
            d * d
        })
        .sum::<f64>()
        / n as f64;
    let stddev_ns = variance.sqrt();

    let domain_str = format!("{:?}", experiment.domain).to_lowercase();
    Ok(ProfileEntry {
        domain: domain_str,
        function_name: experiment.function_name.clone(),
        iterations,
        total_ns: total,
        min_ns,
        max_ns,
        mean_ns,
        stddev_ns,
        median_ns,
    })
}

/// Aggregated profiling report for multiple experiments.
#[derive(Debug, Clone)]
pub struct ProfileReport {
    /// Per-experiment profiling entries.
    pub entries: Vec<ProfileEntry>,
}

impl ProfileReport {
    /// Cumulative total time across all entries.
    pub fn total_time_ns(&self) -> u128 {
        self.entries.iter().map(|e| e.total_ns).sum()
    }

    /// Returns the slowest entry.
    pub fn slowest(&self) -> Option<&ProfileEntry> {
        self.entries.iter().max_by(|a, b| {
            a.mean_ns
                .partial_cmp(&b.mean_ns)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the fastest entry.
    pub fn fastest(&self) -> Option<&ProfileEntry> {
        self.entries.iter().min_by(|a, b| {
            a.mean_ns
                .partial_cmp(&b.mean_ns)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Exports the report as CSV.
    pub fn to_csv(&self) -> String {
        let mut out = String::from(PROFILE_CSV_HEADER);
        out.push('\n');
        for e in &self.entries {
            out.push_str(&e.to_csv_row());
            out.push('\n');
        }
        out
    }

    /// Exports the report as Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = String::from("# Profile Report\n\n");
        out.push_str("| Domain | Function | Iters | Mean (ns) | Min (ns) | Max (ns) | Stddev | Throughput/s |\n");
        out.push_str("|--------|----------|-------|-----------|----------|----------|--------|-------------|\n");
        for e in &self.entries {
            out.push_str(&format!(
                "| {} | {} | {} | {:.0} | {} | {} | {:.0} | {:.0} |\n",
                e.domain,
                e.function_name,
                e.iterations,
                e.mean_ns,
                e.min_ns,
                e.max_ns,
                e.stddev_ns,
                e.throughput_per_sec(),
            ));
        }
        out
    }

    /// Filters entries matching the given `domain`.
    pub fn filter_domain(&self, domain: &str) -> Vec<&ProfileEntry> {
        self.entries.iter().filter(|e| e.domain == domain).collect()
    }
}

/// Profiles a batch of experiments and returns an aggregated report.
pub fn profile_batch(experiments: &[Experiment], iterations: u64) -> ProfileReport {
    let mut entries = Vec::with_capacity(experiments.len());
    for exp in experiments {
        if let Ok(entry) = profile_experiment(exp, iterations) {
            entries.push(entry);
        }
    }
    ProfileReport { entries }
}

/// Shortcut to profile a domain function with scalar parameters.
pub fn quick_profile(
    domain: DomainType,
    func: &str,
    params: Vec<(&str, f64)>,
    iterations: u64,
) -> HubResult<ProfileEntry> {
    let mut exp = Experiment::new(domain, func);
    for (name, val) in params {
        exp = exp.param(name, ParameterValue::Scalar(val));
    }
    profile_experiment(&exp, iterations)
}

/// Computes the ratio of mean times between two profiling entries.
pub fn compare_entries(a: &ProfileEntry, b: &ProfileEntry) -> f64 {
    if b.mean_ns == 0.0 {
        return 0.0;
    }
    a.mean_ns / b.mean_ns
}

/// Formats a duration in nanoseconds into a human-readable unit (ns, µs, ms, s).
pub fn format_ns(ns: f64) -> String {
    if ns >= 1e9 {
        format!("{:.2}s", ns / 1e9)
    } else if ns >= 1e6 {
        format!("{:.2}ms", ns / 1e6)
    } else if ns >= 1e3 {
        format!("{:.2}µs", ns / 1e3)
    } else {
        format!("{:.0}ns", ns)
    }
}

/// Extracts the scalar value from a `RunOutput`, or `None`.
pub fn scalar_value(output: &RunOutput) -> Option<f64> {
    match output {
        RunOutput::Scalar(v) => Some(*v),
        _ => None,
    }
}
