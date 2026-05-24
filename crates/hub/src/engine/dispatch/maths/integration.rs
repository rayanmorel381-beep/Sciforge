//! Dispatch handler for numerical integration functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "halton_sequence_integration" => Ok(RunOutput::Scalar(
            maths::integration::halton_sequence(get_u(p, "index")?, get_u(p, "base")?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
