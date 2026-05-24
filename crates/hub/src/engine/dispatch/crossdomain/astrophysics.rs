//! Dispatch handler for astrophysics functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::cross_domain::astrophysics;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;

/// Dispatches an astrophysics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "schwarzschild_radius" => Ok(RunOutput::Scalar(astrophysics::schwarzschild_radius(
            get_f(p, "mass")?,
        ))),
        "eddington_luminosity" => Ok(RunOutput::Scalar(astrophysics::eddington_luminosity(
            get_f(p, "mass")?,
        ))),
        "chandrasekhar_mass" => Ok(RunOutput::Scalar(astrophysics::chandrasekhar_mass(get_f(
            p, "mu_e",
        )?))),
        "virial_temperature" => Ok(RunOutput::Scalar(astrophysics::virial_temperature(
            get_f(p, "mass")?,
            get_f(p, "radius")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "bondi_accretion_rate" => Ok(RunOutput::Scalar(astrophysics::bondi_accretion_rate(
            get_f(p, "mass")?,
            get_f(p, "density")?,
            get_f(p, "sound_speed")?,
        ))),
        "compton_wavelength" => Ok(RunOutput::Scalar(astrophysics::compton_wavelength(get_f(
            p, "mass",
        )?))),
        "gravitational_redshift" => Ok(RunOutput::Scalar(astrophysics::gravitational_redshift(
            get_f(p, "mass")?,
            get_f(p, "radius")?,
        ))),
        "synchrotron_critical_frequency" => Ok(RunOutput::Scalar(
            astrophysics::synchrotron_critical_frequency(
                get_f(p, "gamma_factor")?,
                get_f(p, "magnetic_field")?,
            ),
        )),
        "photon_sphere_radius" => Ok(RunOutput::Scalar(astrophysics::photon_sphere_radius(
            get_f(p, "mass")?,
        ))),
        "hawking_temperature" => Ok(RunOutput::Scalar(astrophysics::hawking_temperature(get_f(
            p, "mass",
        )?))),
        "relativistic_doppler" => Ok(RunOutput::Scalar(astrophysics::relativistic_doppler(
            get_f(p, "frequency")?,
            get_f(p, "velocity")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "astrophysics: unknown function: {func}"
        ))),
    }
}
