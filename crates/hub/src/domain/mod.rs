//! Scientific domain implementations.
//!
//! Each sub-module re-exports the functions of one discipline (maths,
//! physics, chemistry, biology, astronomy, geology, meteorology) plus
//! shared foundations and cross-domain bridges.

#[cfg(feature = "astronomy")]
pub mod astronomy;
#[cfg(feature = "biology")]
pub mod biology;
#[cfg(feature = "chemistry")]
pub mod chemistry;
pub mod common;
#[cfg(feature = "cross_domain")]
pub mod cross_domain;
#[cfg(feature = "geology")]
pub mod geology;
#[cfg(feature = "maths")]
pub mod maths;
#[cfg(feature = "meteorology")]
pub mod meteorology;
#[cfg(feature = "physics")]
pub mod physics;
