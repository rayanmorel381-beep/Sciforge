//! Scientific domain implementations.
//!
//! Each sub-module re-exports the functions of one discipline (maths,
//! physics, chemistry, biology, astronomy, geology, meteorology) plus
//! shared foundations and cross-domain bridges.

/// Astronomy: orbital mechanics, cosmology, stellar physics, spectroscopy.
pub mod astronomy;
/// Biology: ecology, genetics, physiology, neuroscience, pharmacology.
pub mod biology;
/// Chemistry: thermodynamics, kinetics, electrochemistry, solutions, quantum chemistry.
pub mod chemistry;
/// Shared foundations: physical constants, error types, unit conversions.
pub mod common;
/// Cross-domain modules: astrophysics, biochemistry, geophysics, astrobiology, etc.
pub mod cross_domain;
/// Geology: radiometric dating, seismology, geomorphology, plate tectonics.
pub mod geology;
/// Mathematics: algebra, calculus, statistics, probability, number theory, geometry.
pub mod maths;
/// Meteorology: atmospheric dynamics, thermodynamics, radiation, precipitation.
pub mod meteorology;
/// Physics: classical mechanics, electromagnetism, quantum mechanics, relativity, optics.
pub mod physics;
