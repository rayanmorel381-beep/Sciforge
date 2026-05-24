//! Dispatch handler for quantum physics functions.
//!
//! Delegates to gates, operators, particles, and wavefunctions.

mod gates;
mod operators;
mod particles;
mod wavefunctions;

use crate::domain::common::errors::HubResult;
use crate::engine::dispatch::params::Params;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    wavefunctions::dispatch(func, p)
        .or_else(|_| gates::dispatch(func, p))
        .or_else(|_| operators::dispatch(func, p))
        .or_else(|_| particles::dispatch(func, p))
}
