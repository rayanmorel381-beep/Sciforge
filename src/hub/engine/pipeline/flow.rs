//! Pipeline builder and built-in stages.
//!
//! [`Pipeline`] chains [`Stage`]s that each transform a `Vec<f64>`.
//! Built-in stages include filtering, normalisation, and scaling.

use crate::hub::domain::common::errors::{HubError, HubResult};

/// Named transformation stage.
pub struct Stage {
    /// Stage identifier.
    pub name: String,
    /// Transformation closure.
    pub transform: Box<dyn Fn(Vec<f64>) -> HubResult<Vec<f64>>>,
}

/// Sequential data transformation pipeline.
pub struct Pipeline {
    stages: Vec<Stage>,
}

impl Pipeline {
    /// Creates an empty pipeline.
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    /// Appends a named stage.
    pub fn add_stage(
        mut self,
        name: &str,
        transform: Box<dyn Fn(Vec<f64>) -> HubResult<Vec<f64>>>,
    ) -> Self {
        self.stages.push(Stage {
            name: name.to_string(),
            transform,
        });
        self
    }

    /// Runs all stages in order on the input vector.
    pub fn execute(&self, input: Vec<f64>) -> HubResult<Vec<f64>> {
        let mut data = input;
        for stage in &self.stages {
            data = (stage.transform)(data)
                .map_err(|e| HubError::ComputationFailed(format!("stage '{}': {e}", stage.name)))?;
        }
        Ok(data)
    }

    /// Number of stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Returns the names of all stages in order.
    pub fn stage_names(&self) -> Vec<&str> {
        self.stages.iter().map(|s| s.name.as_str()).collect()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Stage that normalizes values to [0, 1].
pub fn normalize_stage() -> Box<dyn Fn(Vec<f64>) -> HubResult<Vec<f64>>> {
    Box::new(|data| {
        if data.is_empty() {
            return Err(HubError::EmptyData);
        }
        let min = data.iter().copied().fold(f64::INFINITY, f64::min);
        let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;
        if range == 0.0 {
            return Ok(vec![0.0; data.len()]);
        }
        Ok(data.iter().map(|&v| (v - min) / range).collect())
    })
}

/// Stage that scales all values by a constant factor.
pub fn scale_stage(factor: f64) -> Box<dyn Fn(Vec<f64>) -> HubResult<Vec<f64>>> {
    Box::new(move |data| Ok(data.iter().map(|&v| v * factor).collect()))
}

/// Stage that keeps only positive values.
pub fn filter_positive_stage() -> Box<dyn Fn(Vec<f64>) -> HubResult<Vec<f64>>> {
    Box::new(|data| {
        let filtered: Vec<f64> = data.into_iter().filter(|&v| v > 0.0).collect();
        if filtered.is_empty() {
            return Err(HubError::EmptyData);
        }
        Ok(filtered)
    })
}
