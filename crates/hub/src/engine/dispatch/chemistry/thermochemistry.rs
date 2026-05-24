//! Dispatch handler for thermochemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "hess_law" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::hess_law(
                get_v(p, "enthalpies")?,
                get_v(p, "coefficients")?,
            ),
        )),
        "bond_enthalpy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::bond_enthalpy(
                get_v(p, "bonds_broken")?,
                get_v(p, "bonds_formed")?,
            ),
        )),
        "born_haber_lattice_energy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::born_haber_lattice_energy(
                get_f(p, "sublimation")?,
                get_f(p, "ionization")?,
                get_f(p, "dissociation")?,
                get_f(p, "electron_affinity")?,
                get_f(p, "formation")?,
            ),
        )),
        "calorimetry" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::calorimetry(
                get_f(p, "mass")?,
                get_f(p, "specific_heat")?,
                get_f(p, "delta_t")?,
            ),
        )),
        "kirchhoff" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::kirchhoff(
                get_f(p, "delta_h_t1")?,
                get_f(p, "delta_cp")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ),
        )),
        "heat_of_combustion" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::heat_of_combustion(
                get_f(p, "n_co2")?,
                get_f(p, "n_h2o")?,
                get_f(p, "hf_compound")?,
            ),
        )),
        "clausius_clapeyron" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::clausius_clapeyron(
                get_f(p, "p1")?,
                get_f(p, "delta_h_vap")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ),
        )),
        "heat_capacity_ratio" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::heat_capacity_ratio(get_f(p, "cp")?, get_f(p, "cv")?),
        )),
        "adiabatic_flame_temperature" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::adiabatic_flame_temperature(
                get_f(p, "reactant_enthalpy")?,
                get_f(p, "product_enthalpy_base")?,
                get_f(p, "cp_products")?,
                get_f(p, "t_base")?,
            ),
        )),
        "standard_enthalpy_of_reaction" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::standard_enthalpy_of_reaction(
                get_v(p, "products_hf")?,
                get_v(p, "products_coeff")?,
                get_v(p, "reactants_hf")?,
                get_v(p, "reactants_coeff")?,
            ),
        )),
        "bomb_calorimeter" => Ok(RunOutput::Scalar(
            chem::thermochemistry::enthalpy::bomb_calorimeter(
                get_f(p, "delta_t")?,
                get_f(p, "c_cal")?,
                get_f(p, "mass_sample")?,
            ),
        )),

        "entropy_change" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::entropy_change(get_f(p, "q_rev")?, get_f(p, "t")?),
        )),
        "gibbs_free_energy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::gibbs_free_energy(
                get_f(p, "delta_h")?,
                get_f(p, "t")?,
                get_f(p, "delta_s")?,
            ),
        )),
        "spontaneity_temperature" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::spontaneity_temperature(
                get_f(p, "delta_h")?,
                get_f(p, "delta_s")?,
            ),
        )),
        "entropy_mixing_ideal" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::entropy_mixing_ideal(get_v(p, "mole_fractions")?),
        )),
        "gibbs_helmholtz" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::gibbs_helmholtz(
                get_f(p, "delta_g1")?,
                get_f(p, "delta_h")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ),
        )),
        "trouton_rule_entropy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::trouton_rule_entropy(
                get_f(p, "delta_h_vap")?,
                get_f(p, "t_boil")?,
            ),
        )),
        "standard_entropy_of_reaction" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::standard_entropy_of_reaction(
                get_v(p, "products_s")?,
                get_v(p, "products_coeff")?,
                get_v(p, "reactants_s")?,
                get_v(p, "reactants_coeff")?,
            ),
        )),
        "entropy_phase_transition" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::entropy_phase_transition(
                get_f(p, "delta_h")?,
                get_f(p, "t_transition")?,
            ),
        )),
        "debye_entropy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::debye_entropy(get_f(p, "t")?, get_f(p, "theta_d")?),
        )),
        "helmholtz_free_energy" => Ok(RunOutput::Scalar(
            chem::thermochemistry::entropy::helmholtz_free_energy(
                get_f(p, "u")?,
                get_f(p, "t")?,
                get_f(p, "s")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
