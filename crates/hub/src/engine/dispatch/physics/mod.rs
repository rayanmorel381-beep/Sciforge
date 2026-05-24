//! Dispatch handler for physics functions.
//!
//! Delegates to sub-modules: acoustics, electrodynamics, electronics,
//! fluid mechanics, nuclear, optics, quantum, relativity,
//! solid mechanics, and thermodynamics.

mod acoustics;
mod electrodynamics;
mod electronics;
mod fluid_mechanics;
mod nuclear;
mod optics;
mod quantum;
mod relativity;
mod solid_mechanics;
mod thermodynamics;

use super::params::Params;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    acoustics::dispatch(func, p)
        .or_else(|_| electrodynamics::dispatch(func, p))
        .or_else(|_| electronics::dispatch(func, p))
        .or_else(|_| fluid_mechanics::dispatch(func, p))
        .or_else(|_| nuclear::dispatch(func, p))
        .or_else(|_| optics::dispatch(func, p))
        .or_else(|_| quantum::dispatch(func, p))
        .or_else(|_| relativity::dispatch(func, p))
        .or_else(|_| solid_mechanics::dispatch(func, p))
        .or_else(|_| thermodynamics::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("physics: unknown function: {func}")))
}
