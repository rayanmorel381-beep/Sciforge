//! Multi-step experiment campaigns.
//!
//! A [`Campaign`] groups ordered [`Step`]s, each wrapping an
//! [`Experiment`], and collects their results in a [`CampaignResult`].

use super::experience::experiment::Experiment;
use super::experience::runner::{ExperimentRunner, RunOutput};
use crate::domain::common::errors::HubResult;

/// Single step in a campaign.
pub struct Step {
    /// Step name.
    pub name: String,
    /// Experiment to run.
    pub experiment: Experiment,
}

/// Collected results from a campaign run.
pub struct CampaignResult {
    /// Ordered pairs of (step name, output).
    pub step_results: Vec<(String, RunOutput)>,
}

impl CampaignResult {
    /// Returns the output for a step by name.
    pub fn get(&self, name: &str) -> Option<&RunOutput> {
        self.step_results
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, r)| r)
    }

    /// Number of completed steps.
    pub fn len(&self) -> usize {
        self.step_results.len()
    }

    /// Returns `true` if no steps were executed.
    pub fn is_empty(&self) -> bool {
        self.step_results.is_empty()
    }

    /// Extracts all scalar results as (name, value) pairs.
    pub fn scalars(&self) -> Vec<(&str, f64)> {
        self.step_results
            .iter()
            .filter_map(|(n, r)| {
                if let RunOutput::Scalar(v) = r {
                    Some((n.as_str(), *v))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Ordered sequence of experiments executed in series.
pub struct Campaign {
    /// Campaign name.
    pub name: String,
    steps: Vec<Step>,
}

impl Campaign {
    /// Creates a new empty campaign.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: Vec::new(),
        }
    }

    /// Appends a named experiment step.
    pub fn add_step(mut self, name: &str, experiment: Experiment) -> Self {
        self.steps.push(Step {
            name: name.to_string(),
            experiment,
        });
        self
    }

    /// Number of registered steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Runs all steps sequentially and collects results.
    pub fn run(&self, runner: &ExperimentRunner) -> HubResult<CampaignResult> {
        let mut results = Vec::with_capacity(self.steps.len());
        for step in &self.steps {
            let output = runner.run(&step.experiment)?;
            results.push((step.name.clone(), output));
        }
        Ok(CampaignResult {
            step_results: results,
        })
    }
}
