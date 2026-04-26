//! Dispatch handler for chemical equilibrium functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "equilibrium_constant_from_gibbs" => Ok(RunOutput::Scalar(
            chem::equilibrium::constants::equilibrium_constant_from_gibbs(
                get_f(p, "delta_g")?,
                get_f(p, "t")?,
            ),
        )),
        "reaction_quotient" => {
            let pm = get_m(p, "products")?;
            let rm = get_m(p, "reactants")?;
            let products: Vec<(f64, f64)> = pm.iter().map(|r| (r[0], r[1])).collect();
            let reactants: Vec<(f64, f64)> = rm.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                chem::equilibrium::constants::reaction_quotient(&products, &reactants),
            ))
        }
        "le_chatelier_shift" => Ok(RunOutput::Integer(
            chem::equilibrium::constants::le_chatelier_shift(get_f(p, "q")?, get_f(p, "keq")?)
                as i64,
        )),
        "kp_from_kc" => Ok(RunOutput::Scalar(chem::equilibrium::constants::kp_from_kc(
            get_f(p, "kc")?,
            get_f(p, "t")?,
            get_f(p, "delta_n")?,
        ))),
        "vant_hoff" => Ok(RunOutput::Scalar(chem::equilibrium::constants::vant_hoff(
            get_f(p, "k1")?,
            get_f(p, "delta_h")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "degree_of_dissociation" => Ok(RunOutput::Scalar(
            chem::equilibrium::constants::degree_of_dissociation(get_f(p, "keq")?, get_f(p, "c0")?),
        )),
        "temperature_dependence_keq" => Ok(RunOutput::Scalar(
            chem::equilibrium::constants::temperature_dependence_keq(
                get_f(p, "k_ref")?,
                get_f(p, "delta_h")?,
                get_f(p, "t_ref")?,
                get_f(p, "t")?,
            ),
        )),
        "gibbs_from_keq" => Ok(RunOutput::Scalar(
            chem::equilibrium::constants::gibbs_from_keq(get_f(p, "keq")?, get_f(p, "t")?),
        )),
        "pressure_effect_on_keq" => Ok(RunOutput::Scalar(
            chem::equilibrium::constants::pressure_effect_on_keq(
                get_f(p, "keq")?,
                get_f(p, "delta_v")?,
                get_f(p, "p1")?,
                get_f(p, "p2")?,
                get_f(p, "t")?,
            ),
        )),

        "common_ion_effect" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::common_ion_effect(
                get_f(p, "ksp")?,
                get_f(p, "common_ion_conc")?,
                get_f(p, "stoich_coeff")?,
            ),
        )),
        "buffer_capacity" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::buffer_capacity(
                get_f(p, "ca")?,
                get_f(p, "cb")?,
                get_f(p, "ka")?,
                get_f(p, "h")?,
            ),
        )),
        "ionic_ph_weak_acid" => Ok(RunOutput::Scalar(chem::equilibrium::ionic::ph_weak_acid(
            get_f(p, "ka")?,
            get_f(p, "c")?,
        ))),
        "ph_buffer" => Ok(RunOutput::Scalar(chem::equilibrium::ionic::ph_buffer(
            get_f(p, "ka")?,
            get_f(p, "acid")?,
            get_f(p, "base")?,
        ))),
        "solubility_product" => {
            let m = get_m(p, "concentrations")?;
            let pairs: Vec<(f64, f64)> = m.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                chem::equilibrium::ionic::solubility_product(&pairs),
            ))
        }
        "distribution_coefficient" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::distribution_coefficient(
                get_f(p, "c_organic")?,
                get_f(p, "c_aqueous")?,
            ),
        )),
        "solubility_from_ksp" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::solubility_from_ksp(
                get_f(p, "ksp")?,
                get_f(p, "cation_stoich")?,
                get_f(p, "anion_stoich")?,
            ),
        )),
        "formation_constant" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::formation_constant(
                get_f(p, "product_conc")?,
                get_f(p, "metal_conc")?,
                get_f(p, "ligand_conc")?,
                get_f(p, "n")?,
            ),
        )),
        "conditional_formation_constant" => Ok(RunOutput::Scalar(
            chem::equilibrium::ionic::conditional_formation_constant(
                get_f(p, "kf")?,
                get_f(p, "alpha_y")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
