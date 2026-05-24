//! Dispatch handler for green chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "gc_atom_economy" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::atom_economy(
                get_f(p, "mw_product")?,
                get_f(p, "mw_all_products")?,
            ),
        )),
        "e_factor" => Ok(RunOutput::Scalar(chem::green_chemistry::metrics::e_factor(
            get_f(p, "mass_waste")?,
            get_f(p, "mass_product")?,
        ))),
        "process_mass_intensity" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::process_mass_intensity(
                get_f(p, "total_mass_input")?,
                get_f(p, "mass_product")?,
            ),
        )),
        "reaction_mass_efficiency" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::reaction_mass_efficiency(
                get_f(p, "mass_product")?,
                get_f(p, "total_mass_reactants")?,
            ),
        )),
        "carbon_efficiency" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::carbon_efficiency(
                get_f(p, "mass_carbon_product")?,
                get_f(p, "mass_carbon_reactants")?,
            ),
        )),
        "mass_productivity" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::mass_productivity(
                get_f(p, "mass_product")?,
                get_f(p, "total_mass_used")?,
            ),
        )),
        "solvent_intensity" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::solvent_intensity(
                get_f(p, "mass_solvent")?,
                get_f(p, "mass_product")?,
            ),
        )),
        "water_intensity" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::water_intensity(
                get_f(p, "mass_water")?,
                get_f(p, "mass_product")?,
            ),
        )),
        "effective_mass_yield" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::effective_mass_yield(
                get_f(p, "mass_product")?,
                get_f(p, "mass_non_benign")?,
            ),
        )),
        "renewable_feedstock_index" => Ok(RunOutput::Scalar(
            chem::green_chemistry::metrics::renewable_feedstock_index(
                get_f(p, "mass_renewable")?,
                get_f(p, "total_mass")?,
            ),
        )),

        "catalyst_turnover_number" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::catalyst_turnover_number(
                get_f(p, "moles_product")?,
                get_f(p, "moles_catalyst")?,
            ),
        )),
        "catalyst_turnover_frequency" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::catalyst_turnover_frequency(
                get_f(p, "ton")?,
                get_f(p, "time")?,
            ),
        )),
        "energy_efficiency" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::energy_efficiency(
                get_f(p, "useful_energy")?,
                get_f(p, "total_energy")?,
            ),
        )),
        "gc_selectivity" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::selectivity(
                get_f(p, "moles_desired")?,
                get_f(p, "moles_converted")?,
            ),
        )),
        "yield_from_selectivity_conversion" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::yield_from_selectivity_conversion(
                get_f(p, "selectivity_frac")?,
                get_f(p, "conversion_frac")?,
            ),
        )),
        "stoichiometric_factor" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::stoichiometric_factor(
                get_f(p, "actual_mass_reactant")?,
                get_f(p, "stoichiometric_mass")?,
            ),
        )),
        "environmental_quotient" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::environmental_quotient(
                get_f(p, "e_factor")?,
                get_f(p, "unfriendliness")?,
            ),
        )),
        "mass_index" => Ok(RunOutput::Scalar(
            chem::green_chemistry::principles::mass_index(
                get_f(p, "total_mass_input")?,
                get_f(p, "total_mass_output")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
