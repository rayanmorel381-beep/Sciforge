//! Dispatch handler for environmental chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "ozone_formation_rate" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::ozone_formation_rate(
                get_f(p, "k")?,
                get_f(p, "no2")?,
                get_f(p, "hv")?,
            ),
        )),
        "ozone_destruction_rate" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::ozone_destruction_rate(
                get_f(p, "k")?,
                get_f(p, "o3")?,
                get_f(p, "no")?,
            ),
        )),
        "photochemical_smog_potential" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::photochemical_smog_potential(
                get_f(p, "voc")?,
                get_f(p, "nox")?,
            ),
        )),
        "atmospheric_lifetime" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::atmospheric_lifetime(
                get_f(p, "concentration")?,
                get_f(p, "removal_rate")?,
            ),
        )),
        "global_warming_potential" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::global_warming_potential(
                get_f(p, "rf_gas")?,
                get_f(p, "lifetime_gas")?,
                get_f(p, "rf_co2")?,
                get_f(p, "lifetime_co2")?,
            ),
        )),
        "henry_law_solubility" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::henry_law_solubility(
                get_f(p, "kh")?,
                get_f(p, "partial_pressure")?,
            ),
        )),
        "ozone_depletion_potential" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::ozone_depletion_potential(
                get_f(p, "cl_atoms")?,
                get_f(p, "lifetime")?,
                get_f(p, "mw")?,
            ),
        )),
        "photolysis_rate_constant" => Ok(RunOutput::Scalar(
            chem::environmental::atmosphere::photolysis_rate_constant(
                get_f(p, "quantum_yield")?,
                get_f(p, "absorption")?,
                get_f(p, "actinic_flux")?,
            ),
        )),

        "biochemical_oxygen_demand" => Ok(RunOutput::Scalar(
            chem::environmental::water::biochemical_oxygen_demand(
                get_f(p, "bod_ultimate")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "chemical_oxygen_demand" => Ok(RunOutput::Scalar(
            chem::environmental::water::chemical_oxygen_demand(
                get_f(p, "sample_oxygen")?,
                get_f(p, "blank_oxygen")?,
                get_f(p, "volume")?,
            ),
        )),
        "dissolved_oxygen_saturation" => Ok(RunOutput::Scalar(
            chem::environmental::water::dissolved_oxygen_saturation(get_f(p, "t")?),
        )),
        "streeter_phelps" => Ok(RunOutput::Scalar(
            chem::environmental::water::streeter_phelps(
                get_f(p, "d0")?,
                get_f(p, "l0")?,
                get_f(p, "kd")?,
                get_f(p, "kr")?,
                get_f(p, "t")?,
            ),
        )),
        "critical_point_time" => Ok(RunOutput::Scalar(
            chem::environmental::water::critical_point_time(
                get_f(p, "kd")?,
                get_f(p, "kr")?,
                get_f(p, "d0")?,
                get_f(p, "l0")?,
            ),
        )),
        "chlorine_decay" => Ok(RunOutput::Scalar(
            chem::environmental::water::chlorine_decay(
                get_f(p, "c0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "ct_disinfection" => Ok(RunOutput::Scalar(
            chem::environmental::water::ct_disinfection(get_f(p, "c")?, get_f(p, "t")?),
        )),
        "hardness_total" => Ok(RunOutput::Scalar(
            chem::environmental::water::hardness_total(get_f(p, "ca_mg_l")?, get_f(p, "mg_mg_l")?),
        )),
        "langelier_saturation_index" => Ok(RunOutput::Scalar(
            chem::environmental::water::langelier_saturation_index(
                get_f(p, "ph")?,
                get_f(p, "ph_s")?,
            ),
        )),
        "total_dissolved_solids_from_conductivity" => Ok(RunOutput::Scalar(
            chem::environmental::water::total_dissolved_solids_from_conductivity(get_f(
                p,
                "conductivity_us",
            )?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
