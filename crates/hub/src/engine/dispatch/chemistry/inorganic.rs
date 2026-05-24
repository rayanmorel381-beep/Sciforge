//! Dispatch handler for inorganic chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "lattice_energy_born_lande" => Ok(RunOutput::Scalar(
            chem::inorganic::bonding::lattice_energy_born_lande(
                get_f(p, "madelung")?,
                get_f(p, "z_plus")?,
                get_f(p, "z_minus")?,
                get_f(p, "r0")?,
                get_f(p, "born_exponent")?,
            ),
        )),
        "lattice_energy_kapustinskii" => Ok(RunOutput::Scalar(
            chem::inorganic::bonding::lattice_energy_kapustinskii(
                get_i(p, "nu")? as u32,
                get_f(p, "z_plus")?,
                get_f(p, "z_minus")?,
                get_f(p, "r_plus")?,
                get_f(p, "r_minus")?,
            ),
        )),
        "pauling_electronegativity_difference" => Ok(RunOutput::Scalar(
            chem::inorganic::bonding::pauling_electronegativity_difference(
                get_f(p, "en_a")?,
                get_f(p, "en_b")?,
            ),
        )),
        "percent_ionic_character" => Ok(RunOutput::Scalar(
            chem::inorganic::bonding::percent_ionic_character(get_f(p, "en_diff")?),
        )),

        "effective_atomic_number" => Ok(RunOutput::Integer(
            chem::inorganic::coordination::effective_atomic_number(
                get_i(p, "metal_electrons")? as u32,
                get_i(p, "ligand_electrons")? as u32,
            ) as i64,
        )),
        "chelate_effect" => Ok(RunOutput::Scalar(
            chem::inorganic::coordination::chelate_effect(
                get_f(p, "k_mono")?,
                get_f(p, "k_chelate")?,
            ),
        )),
        "irving_williams_stability" => Ok(RunOutput::Scalar(
            chem::inorganic::coordination::irving_williams_stability(
                get_f(p, "ionization_energy")?,
                get_f(p, "ionic_radius")?,
            ),
        )),
        "coordination_number_radius_ratio" => Ok(RunOutput::Integer(
            chem::inorganic::coordination::coordination_number_radius_ratio(
                get_f(p, "r_cation")?,
                get_f(p, "r_anion")?,
            ) as i64,
        )),

        "crystal_field_splitting_octahedral" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::crystal_field_splitting_octahedral(get_f(p, "dq")?),
        )),
        "crystal_field_splitting_tetrahedral" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::crystal_field_splitting_tetrahedral(get_f(
                p, "dq_oct",
            )?),
        )),
        "cfse_octahedral" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::cfse_octahedral(
                get_i(p, "t2g")? as u32,
                get_i(p, "eg")? as u32,
                get_f(p, "dq")?,
                get_f(p, "pairing_energy")?,
            ),
        )),
        "magnetic_moment_spin_only" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::magnetic_moment_spin_only(get_i(p, "unpaired")? as u32),
        )),
        "spectrochemical_series_dq" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::spectrochemical_series_dq(
                get_f(p, "ligand_f")?,
                get_f(p, "metal_g")?,
            ),
        )),
        "jahn_teller_distortion" => Ok(RunOutput::Boolean(
            chem::inorganic::crystal_field::jahn_teller_distortion(
                get_i(p, "eg_occupation")? as u32
            ),
        )),
        "nephelauxetic_ratio" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::nephelauxetic_ratio(
                get_f(p, "b_complex")?,
                get_f(p, "b_free_ion")?,
            ),
        )),
        "racah_parameter_b" => Ok(RunOutput::Scalar(
            chem::inorganic::crystal_field::racah_parameter_b(
                get_v(p, "transitions")?,
                get_f(p, "dq")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
