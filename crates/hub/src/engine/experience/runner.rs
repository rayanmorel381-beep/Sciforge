//! Experiment runner.
//!
//! [`ExperimentRunner`] takes an [`Experiment`], dispatches it to the
//! correct domain handler, and returns a [`RunOutput`] variant (scalar,
//! vector, matrix, text, …).

use super::experiment::Experiment;
use crate::domain::common::errors::HubResult;

/// Typed output of an experiment execution.
#[derive(Debug, Clone)]
pub enum RunOutput {
    Scalar(f64),
    Vector(Vec<f64>),
    Pair(f64, f64),
    Triple(f64, f64, f64),
    Matrix(Vec<Vec<f64>>),
    Boolean(bool),
    Text(String),
    Complex(f64, f64),
    ComplexVector(Vec<(f64, f64)>),
    ComplexMatrix(Vec<Vec<(f64, f64)>>),
    PolynomialOut(Vec<f64>),
    TensorOut {
        data: Vec<f64>,
        shape: Vec<usize>,
    },
    SparseOut {
        rows: usize,
        cols: usize,
        row_ptr: Vec<usize>,
        col_idx: Vec<usize>,
        values: Vec<f64>,
    },
    Integer(i64),
    IntVector(Vec<i64>),
    PairVec(Vec<(f64, f64)>),
    TimeSeries {
        times: Vec<f64>,
        values: Vec<f64>,
    },
}

/// Dispatches experiments to domain handlers and returns results.
pub struct ExperimentRunner;

impl Default for ExperimentRunner {
    fn default() -> Self {
        Self
    }
}

impl ExperimentRunner {
    /// Creates a new runner.
    pub fn new() -> Self {
        Self
    }

    /// Runs the experiment and returns the typed output.
    pub fn run(&self, experiment: &Experiment) -> HubResult<RunOutput> {
        crate::engine::dispatch::dispatch(experiment)
    }
}
