//! Dispatch handler for mathematics functions.
//!
//! Delegates to sub-modules: complex numbers, FFT, graph theory,
//! integration, interpolation, linear algebra, PDEs, probability,
//! signal processing, statistics, tensors, and more.

mod complex;
mod fft;
mod graph;
pub(super) mod helpers;
mod integration;
mod interpolation;
mod linalg;
mod misc;
mod non_euclidean;
mod optimization;
mod pde;
mod polynomial;
mod probability;
mod signal;
mod sparse;
mod statistics;
mod tensor;
mod vector;

use super::params::Params;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    complex::dispatch(func, p)
        .or_else(|_| fft::dispatch(func, p))
        .or_else(|_| graph::dispatch(func, p))
        .or_else(|_| integration::dispatch(func, p))
        .or_else(|_| interpolation::dispatch(func, p))
        .or_else(|_| linalg::dispatch(func, p))
        .or_else(|_| misc::dispatch(func, p))
        .or_else(|_| non_euclidean::dispatch(func, p))
        .or_else(|_| optimization::dispatch(func, p))
        .or_else(|_| pde::dispatch(func, p))
        .or_else(|_| polynomial::dispatch(func, p))
        .or_else(|_| probability::dispatch(func, p))
        .or_else(|_| signal::dispatch(func, p))
        .or_else(|_| sparse::dispatch(func, p))
        .or_else(|_| statistics::dispatch(func, p))
        .or_else(|_| tensor::dispatch(func, p))
        .or_else(|_| vector::dispatch(func, p))
        .map_err(|_| HubError::InvalidInput(format!("maths: unknown function: {func}")))
}
