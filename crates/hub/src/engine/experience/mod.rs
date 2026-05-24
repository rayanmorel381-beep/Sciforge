//! Experiment definition and execution.
//!
//! An [`experiment::Experiment`] binds a domain, function name, and typed
//! parameters; the [`runner::ExperimentRunner`] dispatches it and returns
//! a [`runner::RunOutput`].

/// Experiment struct: domain, function name, typed parameters.
pub mod experiment;
/// ExperimentRunner: executes an Experiment through the dispatch layer.
pub mod runner;
