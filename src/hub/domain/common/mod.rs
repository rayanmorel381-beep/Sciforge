//! Shared foundations for all scientific domains.
//!
//! Physical and mathematical constants, error types, and unit-conversion
//! functions used across every discipline.

/// Fundamental physical and mathematical constants (SI).
pub mod constants;
/// Error types for the Hub computation pipeline.
pub mod errors;
/// Unit systems and conversion functions (length, mass, time, energy, temperature, angle, pressure).
pub mod units;
