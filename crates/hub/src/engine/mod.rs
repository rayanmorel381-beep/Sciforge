//! Computation engine.
//!
//! Dispatches scientific computations, manages experiments and campaigns,
//! provides data-flow pipelines, ODE/PDE simulation, function catalog
//! queries, and async worker infrastructure.

/// Multi-step experiment campaigns with named steps and aggregated results.
pub mod campaign;
/// Domain dispatch: routes (domain, function, params) to the correct scientific function.
pub mod dispatch;
/// Experiment definition and runner: single-shot computations.
pub mod experience;
/// Data-flow pipelines: chainable stages (filter, normalize, scale).
pub mod pipeline;
/// Function catalog and introspection: list available functions per domain.
pub mod query;
/// ODE/PDE simulation: integrators (Euler, RK4, adaptive), dynamical system models.
pub mod simulation;
/// Async worker infrastructure: task queue, scheduler, execution context.
pub mod worker;
