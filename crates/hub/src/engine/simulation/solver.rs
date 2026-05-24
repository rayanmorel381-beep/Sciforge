use super::integrator::{IntegratorConfig, integrate};
use super::model::DynamicalSystem;
use super::result::SimulationResult;
use crate::domain::common::errors::{HubError, HubResult};

/// High-level ODE solver wrapping the integrator.
pub struct Solver {
    config: IntegratorConfig,
}

impl Solver {
    /// Creates a solver with the given integration config.
    pub fn new(config: IntegratorConfig) -> Self {
        Self { config }
    }

    /// Solves the dynamical system and returns the simulation result.
    pub fn solve(&self, system: &dyn DynamicalSystem) -> HubResult<SimulationResult> {
        let dim = system.dimension();
        if dim == 0 {
            return Err(HubError::EmptyData);
        }

        let y0 = system.initial_state();
        let (t0, tf) = system.time_span();

        if tf <= t0 {
            return Err(HubError::InvalidInput("tf must be > t0".into()));
        }

        let (times, states) = integrate(&self.config, system, &y0, t0, tf)?;

        Ok(SimulationResult {
            times,
            states,
            dimension: dim,
            method: self.config.method,
            steps_taken: 0,
        })
    }
}
