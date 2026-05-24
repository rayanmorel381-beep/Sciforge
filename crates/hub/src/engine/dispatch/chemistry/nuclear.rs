//! Dispatch handler for nuclear chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "half_life_to_decay_constant" => Ok(RunOutput::Scalar(
            chem::nuclear::decay::half_life_to_decay_constant(get_f(p, "half_life")?),
        )),
        "decay_constant_to_half_life" => Ok(RunOutput::Scalar(
            chem::nuclear::decay::decay_constant_to_half_life(get_f(p, "lambda")?),
        )),
        "remaining_nuclei" => Ok(RunOutput::Scalar(chem::nuclear::decay::remaining_nuclei(
            get_f(p, "n0")?,
            get_f(p, "lambda")?,
            get_f(p, "t")?,
        ))),
        "activity" => Ok(RunOutput::Scalar(chem::nuclear::decay::activity(
            get_f(p, "lambda")?,
            get_f(p, "n")?,
        ))),
        "decay_chain_intermediate" => Ok(RunOutput::Scalar(
            chem::nuclear::decay::decay_chain_intermediate(
                get_f(p, "n0")?,
                get_f(p, "lambda1")?,
                get_f(p, "lambda2")?,
                get_f(p, "t")?,
            ),
        )),
        "specific_activity" => Ok(RunOutput::Scalar(chem::nuclear::decay::specific_activity(
            get_f(p, "lambda")?,
            get_f(p, "avogadro")?,
            get_f(p, "molar_mass")?,
        ))),

        "mass_defect" => Ok(RunOutput::Scalar(chem::nuclear::energy::mass_defect(
            get_i(p, "z")? as u32,
            get_i(p, "n")? as u32,
            get_f(p, "atomic_mass")?,
        ))),
        "binding_energy" => Ok(RunOutput::Scalar(chem::nuclear::energy::binding_energy(
            get_f(p, "mass_defect")?,
        ))),
        "binding_energy_per_nucleon" => Ok(RunOutput::Scalar(
            chem::nuclear::energy::binding_energy_per_nucleon(
                get_f(p, "binding_energy")?,
                get_i(p, "a")? as u32,
            ),
        )),
        "q_value" => Ok(RunOutput::Scalar(chem::nuclear::energy::q_value(
            get_v(p, "reactant_masses")?,
            get_v(p, "product_masses")?,
        ))),
        "semi_empirical_mass_formula" => Ok(RunOutput::Scalar(
            chem::nuclear::energy::semi_empirical_mass_formula(
                get_i(p, "a")? as u32,
                get_i(p, "z")? as u32,
                get_f(p, "av")?,
                get_f(p, "as_")?,
                get_f(p, "ac")?,
                get_f(p, "aa")?,
                get_f(p, "ap")?,
            ),
        )),
        "threshold_energy" => Ok(RunOutput::Scalar(chem::nuclear::energy::threshold_energy(
            get_f(p, "q")?,
            get_f(p, "mass_projectile")?,
            get_f(p, "mass_target")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
