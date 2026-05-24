//! Dispatch handler for stoichiometry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "limiting_reagent" => Ok(RunOutput::Integer(
            chem::stoichiometry::balancing::limiting_reagent(
                get_v(p, "moles")?,
                get_v(p, "coefficients")?,
            ) as i64,
        )),
        "theoretical_yield" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::theoretical_yield(
                get_f(p, "moles_limiting")?,
                get_f(p, "coeff_limiting")?,
                get_f(p, "coeff_product")?,
                get_f(p, "mw_product")?,
            ),
        )),
        "percent_yield" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::percent_yield(
                get_f(p, "actual")?,
                get_f(p, "theoretical")?,
            ),
        )),
        "excess_reagent" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::excess_reagent(
                get_f(p, "moles_a")?,
                get_f(p, "coeff_a")?,
                get_f(p, "moles_b")?,
                get_f(p, "coeff_b")?,
            ),
        )),
        "moles_from_mass" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::moles_from_mass(
                get_f(p, "mass")?,
                get_f(p, "molar_mass")?,
            ),
        )),
        "mass_from_moles" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::mass_from_moles(
                get_f(p, "moles")?,
                get_f(p, "molar_mass")?,
            ),
        )),
        "number_of_particles" => Ok(RunOutput::Scalar(
            chem::stoichiometry::balancing::number_of_particles(get_f(p, "moles")?),
        )),

        "oxidation_number_simple" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::oxidation_number_simple(
                get_i(p, "charge")? as i32,
                get_i(p, "num_atoms")? as u32,
            ),
        )),
        "equivalent_mass" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::equivalent_mass(
                get_f(p, "molar_mass")?,
                get_f(p, "n_equivalents")?,
            ),
        )),
        "normality" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::normality(
                get_f(p, "equivalents")?,
                get_f(p, "volume_liters")?,
            ),
        )),
        "atom_economy" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::atom_economy(
                get_f(p, "mw_desired_product")?,
                get_v(p, "mw_all_products")?,
            ),
        )),
        "empirical_formula_ratio" => Ok(RunOutput::Vector(
            chem::stoichiometry::calculations::empirical_formula_ratio(
                get_v(p, "masses")?,
                get_v(p, "molar_masses")?,
            ),
        )),
        "dilution_factor" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::dilution_factor(
                get_f(p, "v_initial")?,
                get_f(p, "v_final")?,
            ),
        )),
        "stoichiometric_ratio" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::stoichiometric_ratio(
                get_f(p, "coeff_a")?,
                get_f(p, "coeff_b")?,
            ),
        )),
        "mass_percent" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::mass_percent(
                get_f(p, "mass_solute")?,
                get_f(p, "mass_solution")?,
            ),
        )),
        "ppm_from_mass" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::ppm_from_mass(
                get_f(p, "mass_solute")?,
                get_f(p, "mass_solution")?,
            ),
        )),
        "molarity_to_molality" => Ok(RunOutput::Scalar(
            chem::stoichiometry::calculations::molarity_to_molality(
                get_f(p, "molarity")?,
                get_f(p, "density")?,
                get_f(p, "molar_mass_solute")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
