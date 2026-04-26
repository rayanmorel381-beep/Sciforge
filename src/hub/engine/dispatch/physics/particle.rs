//! Dispatch handler for particle physics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "planck_energy" => Ok(RunOutput::Scalar(phys::particle::planck_energy())),
        "planck_density" => Ok(RunOutput::Scalar(phys::particle::planck_density())),
        "planck_force" => Ok(RunOutput::Scalar(phys::particle::planck_force())),
        "planck_pressure" => Ok(RunOutput::Scalar(phys::particle::planck_pressure())),
        "planck_temperature" => Ok(RunOutput::Scalar(phys::particle::planck_temperature())),
        "planck_charge" => Ok(RunOutput::Scalar(phys::particle::planck_charge())),
        "planck_impedance" => Ok(RunOutput::Scalar(phys::particle::planck_impedance())),
        "planck_angular_frequency" => {
            Ok(RunOutput::Scalar(phys::particle::planck_angular_frequency()))
        }
        "schwarzschild_radius_planck" => Ok(RunOutput::Scalar(
            phys::particle::schwarzschild_radius_planck(get_f(p, "mass_planck_units")?),
        )),
        "hawking_temperature" => Ok(RunOutput::Scalar(phys::particle::hawking_temperature(
            get_f(p, "mass")?,
        ))),
        "hawking_luminosity" => Ok(RunOutput::Scalar(phys::particle::hawking_luminosity(
            get_f(p, "mass")?,
        ))),
        "hawking_evaporation_time" => Ok(RunOutput::Scalar(
            phys::particle::hawking_evaporation_time(get_f(p, "mass")?),
        )),
        "unruh_temperature" => Ok(RunOutput::Scalar(phys::particle::unruh_temperature(get_f(
            p,
            "acceleration",
        )?))),
        "fermi_coupling_constant" => {
            Ok(RunOutput::Scalar(phys::particle::fermi_coupling_constant()))
        }
        "weak_decay_rate" => Ok(RunOutput::Scalar(phys::particle::weak_decay_rate(get_f(
            p,
            "energy_gev",
        )?))),
        "muon_decay_width" => Ok(RunOutput::Scalar(phys::particle::muon_decay_width())),
        "fine_structure_constant" => {
            Ok(RunOutput::Scalar(phys::particle::fine_structure_constant()))
        }
        "strong_coupling_constant" => {
            Ok(RunOutput::Scalar(phys::particle::strong_coupling_constant()))
        }
        "qcd_running_coupling" => Ok(RunOutput::Scalar(phys::particle::qcd_running_coupling(
            get_f(p, "q_gev")?,
        ))),
        "electromagnetic_coupling_running" => Ok(RunOutput::Scalar(
            phys::particle::electromagnetic_coupling_running(get_f(p, "q_gev")?),
        )),
        "weak_mixing_angle" => Ok(RunOutput::Scalar(phys::particle::weak_mixing_angle())),
        "w_boson_mass_gev" => Ok(RunOutput::Scalar(phys::particle::w_boson_mass_gev())),
        "z_boson_mass_gev" => Ok(RunOutput::Scalar(phys::particle::z_boson_mass_gev())),
        "higgs_vev_gev" => Ok(RunOutput::Scalar(phys::particle::higgs_vev_gev())),
        "compton_time" => Ok(RunOutput::Scalar(phys::particle::compton_time(get_f(
            p, "mass",
        )?))),
        "gravitational_coupling" => Ok(RunOutput::Scalar(phys::particle::gravitational_coupling(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "photon_energy" => Ok(RunOutput::Scalar(phys::particle::photon_energy(get_f(
            p,
            "frequency",
        )?))),
        "photon_momentum" => Ok(RunOutput::Scalar(phys::particle::photon_momentum(get_f(
            p,
            "frequency",
        )?))),
        "pair_production_threshold_energy" => Ok(RunOutput::Scalar(
            phys::particle::pair_production_threshold_energy(),
        )),
        "cross_section_thomson" => Ok(RunOutput::Scalar(phys::particle::cross_section_thomson())),
        "neutrino_mass_upper_bound" => Ok(RunOutput::Scalar(
            phys::particle::neutrino_mass_upper_bound(),
        )),
        "neutrino_de_broglie_wavelength" => Ok(RunOutput::Scalar(
            phys::particle::neutrino_de_broglie_wavelength(get_f(p, "energy_ev")?),
        )),
        "classical_electron_radius" => Ok(RunOutput::Scalar(
            phys::particle::classical_electron_radius(),
        )),
        "bohr_velocity" => Ok(RunOutput::Scalar(phys::particle::bohr_velocity())),
        "schwinger_critical_field" => {
            Ok(RunOutput::Scalar(phys::particle::schwinger_critical_field()))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
