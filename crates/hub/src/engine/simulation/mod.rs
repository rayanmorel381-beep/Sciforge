//! Numerical simulation engine: integration, dynamical models, solver, and results.

/// Numerical integration methods (Euler, RK4, Dormand-Prince).
pub mod integrator;
/// Dynamical system trait and built-in models (harmonic oscillator, Lotka-Volterra).
pub mod model;
/// Time-series results container for simulation outputs.
pub mod result;
/// High-level solver interface wrapping integrator + model.
pub mod solver;
