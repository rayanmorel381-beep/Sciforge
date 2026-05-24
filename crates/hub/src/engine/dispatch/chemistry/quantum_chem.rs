//! Dispatch handler for quantum chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "huckel_secular_determinant" => Ok(RunOutput::Matrix(
            chem::quantum_chem::huckel::huckel_secular_determinant(
                get_u(p, "n")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "huckel_cyclic_determinant" => Ok(RunOutput::Matrix(
            chem::quantum_chem::huckel::huckel_cyclic_determinant(
                get_u(p, "n")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "huckel_charge_density" => Ok(RunOutput::Vector(
            chem::quantum_chem::huckel::huckel_charge_density(
                get_m(p, "coefficients")?,
                get_v(p, "occupations")?,
            ),
        )),
        "huckel_bond_order" => Ok(RunOutput::Scalar(
            chem::quantum_chem::huckel::huckel_bond_order(
                get_m(p, "coefficients")?,
                get_v(p, "occupations")?,
                get_u(p, "atom_i")?,
                get_u(p, "atom_j")?,
            ),
        )),
        "huckel_total_pi_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::huckel::huckel_total_pi_energy(
                get_v(p, "eigenvalues")?,
                get_v(p, "occupations")?,
            ),
        )),
        "huckel_free_valence" => Ok(RunOutput::Scalar(
            chem::quantum_chem::huckel::huckel_free_valence(get_f(p, "bond_orders_sum")?),
        )),

        "lcao_bonding_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::lcao_bonding_energy(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "overlap")?,
            ),
        )),
        "lcao_antibonding_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::lcao_antibonding_energy(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "overlap")?,
            ),
        )),
        "orbital_overlap_integral_1s" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::overlap_integral_1s(get_f(p, "r")?, get_f(p, "zeta")?),
        )),
        "orbital_hartree_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::hartree_energy(
                get_f(p, "kinetic")?,
                get_f(p, "nuclear_attraction")?,
                get_f(p, "electron_repulsion")?,
            ),
        )),
        "roothaan_total_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::roothaan_total_energy(
                get_v(p, "one_electron")?,
                get_v(p, "fock_eigenvalues")?,
                get_f(p, "nuclear_repulsion")?,
            ),
        )),
        "mulliken_population" => Ok(RunOutput::Vector(
            chem::quantum_chem::orbitals::mulliken_population(
                get_m(p, "density")?,
                get_m(p, "overlap")?,
            ),
        )),
        "nuclear_repulsion_energy" => Ok(RunOutput::Scalar(
            chem::quantum_chem::orbitals::nuclear_repulsion_energy(
                get_f(p, "z1")?,
                get_f(p, "z2")?,
                get_f(p, "r")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
