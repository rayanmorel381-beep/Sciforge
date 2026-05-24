use super::integrator::IntegrationMethod;

/// Output of a simulation run.
pub struct SimulationResult {
    /// Time points.
    pub times: Vec<f64>,
    /// State vectors at each time point.
    pub states: Vec<Vec<f64>>,
    /// Number of state dimensions.
    pub dimension: usize,
    /// Integration method used.
    pub method: IntegrationMethod,
    /// Number of steps taken.
    pub steps_taken: usize,
}

impl SimulationResult {
    /// Returns the last state vector.
    pub fn final_state(&self) -> Option<&[f64]> {
        self.states.last().map(|v| v.as_slice())
    }

    /// Returns the state vector at the given index.
    pub fn state_at(&self, index: usize) -> Option<&[f64]> {
        self.states.get(index).map(|v| v.as_slice())
    }

    /// Returns the time at the given index.
    pub fn time_at(&self, index: usize) -> Option<f64> {
        self.times.get(index).copied()
    }

    /// Number of recorded time steps.
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Returns `true` if no steps were recorded.
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    /// Extracts the time series for a single dimension.
    pub fn column(&self, dim_index: usize) -> Vec<f64> {
        self.states
            .iter()
            .filter_map(|s| s.get(dim_index).copied())
            .collect()
    }

    /// Returns the maximum value in the given dimension.
    pub fn max_of(&self, dim_index: usize) -> f64 {
        self.column(dim_index)
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Returns the minimum value in the given dimension.
    pub fn min_of(&self, dim_index: usize) -> f64 {
        self.column(dim_index)
            .into_iter()
            .fold(f64::INFINITY, f64::min)
    }
}
