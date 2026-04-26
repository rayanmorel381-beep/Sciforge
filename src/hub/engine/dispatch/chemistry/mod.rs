//! Dispatch handler for chemistry functions.
//!
//! Delegates to sub-modules covering acid-base, kinetics,
//! electrochemistry, spectroscopy, thermochemistry, and more.

mod acid_base;
mod analytical;
mod colloids;
mod computational;
mod crystallography;
mod electrochemistry;
mod environmental;
mod equilibrium;
mod gas_laws;
mod green_chemistry;
mod inorganic;
mod kinetics;
mod molecular;
mod nuclear;
mod organic;
mod photochemistry;
mod polymers;
mod quantum_chem;
mod reaction_engineering;
mod solid_state;
mod solutions;
mod spectroscopy;
mod stoichiometry;
mod surface;
mod thermochemistry;
mod transport;

use super::params::Params;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    acid_base::dispatch(func, p)
        .or_else(|_| analytical::dispatch(func, p))
        .or_else(|_| colloids::dispatch(func, p))
        .or_else(|_| computational::dispatch(func, p))
        .or_else(|_| crystallography::dispatch(func, p))
        .or_else(|_| electrochemistry::dispatch(func, p))
        .or_else(|_| environmental::dispatch(func, p))
        .or_else(|_| equilibrium::dispatch(func, p))
        .or_else(|_| gas_laws::dispatch(func, p))
        .or_else(|_| green_chemistry::dispatch(func, p))
        .or_else(|_| inorganic::dispatch(func, p))
        .or_else(|_| kinetics::dispatch(func, p))
        .or_else(|_| molecular::dispatch(func, p))
        .or_else(|_| nuclear::dispatch(func, p))
        .or_else(|_| organic::dispatch(func, p))
        .or_else(|_| photochemistry::dispatch(func, p))
        .or_else(|_| polymers::dispatch(func, p))
        .or_else(|_| quantum_chem::dispatch(func, p))
        .or_else(|_| reaction_engineering::dispatch(func, p))
        .or_else(|_| solid_state::dispatch(func, p))
        .or_else(|_| solutions::dispatch(func, p))
        .or_else(|_| spectroscopy::dispatch(func, p))
        .or_else(|_| stoichiometry::dispatch(func, p))
        .or_else(|_| surface::dispatch(func, p))
        .or_else(|_| thermochemistry::dispatch(func, p))
        .or_else(|_| transport::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("chemistry: unknown function: {func}")))
}
