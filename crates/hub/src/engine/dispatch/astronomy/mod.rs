//! Dispatch handler for astronomy functions.
//!
//! Delegates to sub-modules by topic: celestial mechanics, cosmology,
//! galactic dynamics, impacts, orbits, rotation, stellar physics, tides.

mod celestial;
mod cosmology;
mod galactic;
mod impacts;
mod orbits;
mod rotation;
mod stellar;
mod tides;

use super::params::Params;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    celestial::dispatch(func, p)
        .or_else(|_| cosmology::dispatch(func, p))
        .or_else(|_| galactic::dispatch(func, p))
        .or_else(|_| impacts::dispatch(func, p))
        .or_else(|_| orbits::dispatch(func, p))
        .or_else(|_| rotation::dispatch(func, p))
        .or_else(|_| stellar::dispatch(func, p))
        .or_else(|_| tides::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("astronomy: unknown function: {func}")))
}
