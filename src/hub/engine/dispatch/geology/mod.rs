//! Dispatch handler for geology functions.
//!
//! Delegates to sub-modules: radiometric dating, erosion, glaciology,
//! hydrology, petrology, seismology, and tectonics.

mod dating;
mod erosion;
mod glaciology;
mod hydrology;
mod petrology;
mod seismology;
mod tectonics;

use super::params::Params;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    dating::dispatch(func, p)
        .or_else(|_| erosion::dispatch(func, p))
        .or_else(|_| glaciology::dispatch(func, p))
        .or_else(|_| hydrology::dispatch(func, p))
        .or_else(|_| petrology::dispatch(func, p))
        .or_else(|_| seismology::dispatch(func, p))
        .or_else(|_| tectonics::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("geology: unknown function: {func}")))
}
