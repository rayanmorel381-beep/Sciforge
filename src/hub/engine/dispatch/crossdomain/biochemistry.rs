//! Dispatch handler for biochemistry functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::biochemistry;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches a biochemistry function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "michaelis_menten_rate" => Ok(RunOutput::Scalar(biochemistry::michaelis_menten_rate(
            get_f(p, "vmax")?,
            get_f(p, "substrate")?,
            get_f(p, "km")?,
        ))),
        "henderson_hasselbalch" => Ok(RunOutput::Scalar(biochemistry::henderson_hasselbalch(
            get_f(p, "pka")?,
            get_f(p, "base_conc")?,
            get_f(p, "acid_conc")?,
        ))),
        "gibbs_free_energy" => Ok(RunOutput::Scalar(biochemistry::gibbs_free_energy(
            get_f(p, "delta_h")?,
            get_f(p, "delta_s")?,
            get_f(p, "temperature")?,
        ))),
        "nernst_potential" => Ok(RunOutput::Scalar(biochemistry::nernst_potential(
            get_f(p, "z")?,
            get_f(p, "c_out")?,
            get_f(p, "c_in")?,
            get_f(p, "temperature")?,
        ))),
        "enzyme_turnover_number" => Ok(RunOutput::Scalar(biochemistry::enzyme_turnover_number(
            get_f(p, "vmax")?,
            get_f(p, "enzyme_conc")?,
        ))),
        "competitive_inhibition_rate" => Ok(RunOutput::Scalar(
            biochemistry::competitive_inhibition_rate(
                get_f(p, "vmax")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
                get_f(p, "inhibitor")?,
                get_f(p, "ki")?,
            ),
        )),
        "osmotic_pressure" => Ok(RunOutput::Scalar(biochemistry::osmotic_pressure(
            get_f(p, "molarity")?,
            get_f(p, "temperature")?,
            get_f(p, "i_factor")?,
        ))),
        "arrhenius_rate" => Ok(RunOutput::Scalar(biochemistry::arrhenius_rate(
            get_f(p, "prefactor")?,
            get_f(p, "activation_energy")?,
            get_f(p, "temperature")?,
        ))),
        "binding_free_energy" => Ok(RunOutput::Scalar(biochemistry::binding_free_energy(
            get_f(p, "kd")?,
            get_f(p, "temperature")?,
        ))),
        "cooperativity_hill" => Ok(RunOutput::Scalar(biochemistry::cooperativity_hill(
            get_f(p, "substrate")?,
            get_f(p, "k_half")?,
            get_f(p, "n_hill")?,
        ))),
        "ph_from_concentration" => Ok(RunOutput::Scalar(biochemistry::ph_from_concentration(
            get_f(p, "h_concentration")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "biochemistry: unknown function: {func}"
        ))),
    }
}
