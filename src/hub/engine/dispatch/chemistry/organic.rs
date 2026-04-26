//! Dispatch handler for organic chemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "cahn_ingold_prelog_priority" => {
            let v = get_v(p, "atomic_numbers")?;
            let an: Vec<u32> = v.iter().map(|&x| x as u32).collect();
            Ok(RunOutput::IntVector(
                chem::organic::descriptors::cahn_ingold_prelog_priority(&an)
                    .iter()
                    .map(|&x| x as i64)
                    .collect(),
            ))
        }
        "topological_index_wiener" => {
            let m = get_m(p, "distance_matrix")?;
            let dm: Vec<Vec<u32>> = m
                .iter()
                .map(|r| r.iter().map(|&x| x as u32).collect())
                .collect();
            Ok(RunOutput::Integer(
                chem::organic::descriptors::topological_index_wiener(&dm) as i64,
            ))
        }
        "randic_index" => {
            let m = get_m(p, "adjacency")?;
            let adj: Vec<Vec<bool>> = m
                .iter()
                .map(|r| r.iter().map(|&x| x != 0.0).collect())
                .collect();
            let dv = get_v(p, "degrees")?;
            let deg: Vec<u32> = dv.iter().map(|&x| x as u32).collect();
            Ok(RunOutput::Scalar(chem::organic::descriptors::randic_index(
                &adj, &deg,
            )))
        }
        "partition_coefficient_log_p" => Ok(RunOutput::Scalar(
            chem::organic::descriptors::partition_coefficient_log_p(get_v(p, "fragments")?),
        )),
        "topological_polar_surface_area" => Ok(RunOutput::Scalar(
            chem::organic::descriptors::topological_polar_surface_area(get_v(p, "contributions")?),
        )),
        "rotatable_bonds" => Ok(RunOutput::Integer(
            chem::organic::descriptors::rotatable_bonds(
                get_i(p, "single_bonds")? as u32,
                get_i(p, "ring_bonds")? as u32,
                get_i(p, "terminal_bonds")? as u32,
            ) as i64,
        )),

        "sn1_rate" => Ok(RunOutput::Scalar(chem::organic::reactions::sn1_rate(
            get_f(p, "k")?,
            get_f(p, "substrate")?,
        ))),
        "sn2_rate" => Ok(RunOutput::Scalar(chem::organic::reactions::sn2_rate(
            get_f(p, "k")?,
            get_f(p, "substrate")?,
            get_f(p, "nucleophile")?,
        ))),
        "e1_rate" => Ok(RunOutput::Scalar(chem::organic::reactions::e1_rate(
            get_f(p, "k")?,
            get_f(p, "substrate")?,
        ))),
        "e2_rate" => Ok(RunOutput::Scalar(chem::organic::reactions::e2_rate(
            get_f(p, "k")?,
            get_f(p, "substrate")?,
            get_f(p, "base")?,
        ))),
        "hammett_equation" => Ok(RunOutput::Scalar(
            chem::organic::reactions::hammett_equation(
                get_f(p, "rho")?,
                get_f(p, "sigma")?,
                get_f(p, "log_k0")?,
            ),
        )),
        "hammett_acidity" => Ok(RunOutput::Scalar(
            chem::organic::reactions::hammett_acidity(
                get_f(p, "pka_ref")?,
                get_f(p, "rho")?,
                get_f(p, "sigma")?,
            ),
        )),
        "taft_equation" => Ok(RunOutput::Scalar(chem::organic::reactions::taft_equation(
            get_f(p, "rho_star")?,
            get_f(p, "sigma_star")?,
            get_f(p, "es")?,
            get_f(p, "delta")?,
        ))),

        "degree_of_unsaturation" => Ok(RunOutput::Scalar(
            chem::organic::structure::degree_of_unsaturation(
                get_i(p, "c")? as u32,
                get_i(p, "h")? as u32,
                get_i(p, "n")? as u32,
                get_i(p, "halogens")? as u32,
            ),
        )),
        "molecular_formula_mass" => Ok(RunOutput::Scalar(
            chem::organic::structure::molecular_formula_mass(
                get_i(p, "c")? as u32,
                get_i(p, "h")? as u32,
                get_i(p, "o")? as u32,
                get_i(p, "n")? as u32,
                get_i(p, "s")? as u32,
            ),
        )),
        "alkane_boiling_point_estimate" => Ok(RunOutput::Scalar(
            chem::organic::structure::alkane_boiling_point_estimate(
                get_i(p, "carbon_count")? as u32
            ),
        )),
        "huckel_energy_linear" => Ok(RunOutput::Vector(
            chem::organic::structure::huckel_energy_linear(
                get_u(p, "n_atoms")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "huckel_energy_cyclic" => Ok(RunOutput::Vector(
            chem::organic::structure::huckel_energy_cyclic(
                get_u(p, "n_atoms")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "delocalization_energy" => Ok(RunOutput::Scalar(
            chem::organic::structure::delocalization_energy(
                get_f(p, "total_pi_energy")?,
                get_f(p, "isolated_energy")?,
            ),
        )),
        "resonance_stabilization" => Ok(RunOutput::Scalar(
            chem::organic::structure::resonance_stabilization(get_u(p, "n_structures")?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
