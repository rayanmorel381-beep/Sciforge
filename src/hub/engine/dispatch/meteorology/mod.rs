//! Dispatch handler for meteorology functions.
//!
//! Delegates to sub-modules: atmosphere, dynamics, ocean coupling,
//! precipitation, radiation, storms, and winds.

mod atmosphere;
mod dynamics;
mod ocean;
mod precipitation;
mod radiation;
mod storms;
mod winds;

use super::params::Params;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    atmosphere::dispatch(func, p)
        .or_else(|_| dynamics::dispatch(func, p))
        .or_else(|_| ocean::dispatch(func, p))
        .or_else(|_| precipitation::dispatch(func, p))
        .or_else(|_| radiation::dispatch(func, p))
        .or_else(|_| storms::dispatch(func, p))
        .or_else(|_| winds::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("meteorology: unknown function: {func}")))
}
