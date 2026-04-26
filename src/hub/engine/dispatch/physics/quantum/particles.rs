//! Dispatch handler for quantum particle functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

pub(crate) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "quantum_bohr_radius_nth" => Ok(RunOutput::Scalar(phys::quantum::bohr_radius_nth(
            get_i(p, "n")? as u32,
            get_f(p, "z_eff")?,
        ))),
        "hydrogen_energy_level" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_energy_level(
            get_i(p, "n")? as u32,
        ))),
        "hydrogen_energy_level_z" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_energy_level_z(
            get_i(p, "n")? as u32,
            get_f(p, "z")?,
        ))),
        "rydberg_wavelength" => Ok(RunOutput::Scalar(phys::quantum::rydberg_wavelength(
            get_i(p, "n1")? as u32,
            get_i(p, "n2")? as u32,
        ))),
        "quantum_compton_wavelength_shift" => Ok(RunOutput::Scalar(
            phys::quantum::compton_wavelength_shift(get_f(p, "theta")?),
        )),
        "quantum_cyclotron_frequency" => Ok(RunOutput::Scalar(phys::quantum::cyclotron_frequency(
            get_f(p, "b_field")?,
        ))),
        "cyclotron_frequency_particle" => Ok(RunOutput::Scalar(
            phys::quantum::cyclotron_frequency_particle(
                get_f(p, "charge")?,
                get_f(p, "mass")?,
                get_f(p, "b_field")?,
            ),
        )),
        "quantum_larmor_radius" => Ok(RunOutput::Scalar(phys::quantum::larmor_radius(
            get_f(p, "v_perp")?,
            get_f(p, "b_field")?,
        ))),
        "larmor_radius_particle" => Ok(RunOutput::Scalar(phys::quantum::larmor_radius_particle(
            get_f(p, "mass")?,
            get_f(p, "v_perp")?,
            get_f(p, "charge")?,
            get_f(p, "b_field")?,
        ))),
        "nuclear_zeeman_splitting" => Ok(RunOutput::Scalar(
            phys::quantum::nuclear_zeeman_splitting(get_f(p, "m_i")?, get_f(p, "b_field")?),
        )),
        "anomalous_zeeman_splitting" => Ok(RunOutput::Scalar(
            phys::quantum::anomalous_zeeman_splitting(
                get_f(p, "m_j")?,
                get_f(p, "g_j")?,
                get_f(p, "b_field")?,
            ),
        )),
        "muonic_hydrogen_energy" => Ok(RunOutput::Scalar(phys::quantum::muonic_hydrogen_energy(
            get_i(p, "n")? as u32,
        ))),
        "muonic_bohr_radius" => Ok(RunOutput::Scalar(phys::quantum::muonic_bohr_radius())),
        "tau_lepton_mass" => Ok(RunOutput::Scalar(phys::quantum::tau_lepton_mass())),
        "muon_mass_kg" => Ok(RunOutput::Scalar(phys::quantum::muon_mass_kg())),
        "proton_gyromagnetic_ratio" => Ok(RunOutput::Scalar(
            phys::quantum::proton_gyromagnetic_ratio(get_f(p, "b_field")?),
        )),
        "neutron_mass" => Ok(RunOutput::Scalar(phys::quantum::neutron_mass())),
        "proton_mass" => Ok(RunOutput::Scalar(phys::quantum::proton_mass())),
        "reduced_mass" => Ok(RunOutput::Scalar(phys::quantum::reduced_mass(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "de_broglie_wavelength" => Ok(RunOutput::Scalar(phys::quantum::de_broglie_wavelength(
            get_f(p, "mass")?,
            get_f(p, "velocity")?,
        ))),
        "magnetic_moment_orbital" => Ok(RunOutput::Scalar(phys::quantum::magnetic_moment_orbital(
            get_i(p, "m_l")? as i32,
        ))),
        "fine_structure_splitting" => {
            Ok(RunOutput::Scalar(phys::quantum::fine_structure_splitting(
                get_i(p, "n")? as u32,
                get_f(p, "j")?,
                get_f(p, "z")?,
            )))
        }
        "hyperfine_splitting_hydrogen" => Ok(RunOutput::Scalar(
            phys::quantum::hyperfine_splitting_hydrogen(),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
