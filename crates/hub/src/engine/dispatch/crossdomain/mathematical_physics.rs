//! Dispatch handler for mathematical physics functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::cross_domain::mathematical_physics;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;

/// Dispatches a mathematical physics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "de_broglie_wavelength" => Ok(RunOutput::Scalar(
            mathematical_physics::de_broglie_wavelength(get_f(p, "momentum")?),
        )),
        "heisenberg_uncertainty_position" => Ok(RunOutput::Scalar(
            mathematical_physics::heisenberg_uncertainty_position(get_f(p, "delta_p")?),
        )),
        "heisenberg_uncertainty_momentum" => Ok(RunOutput::Scalar(
            mathematical_physics::heisenberg_uncertainty_momentum(get_f(p, "delta_x")?),
        )),
        "wkb_tunneling_probability" => Ok(RunOutput::Scalar(
            mathematical_physics::wkb_tunneling_probability(
                get_f(p, "energy")?,
                get_f(p, "potential")?,
                get_f(p, "barrier_width")?,
                get_f(p, "particle_mass")?,
            ),
        )),
        "partition_function_harmonic" => Ok(RunOutput::Scalar(
            mathematical_physics::partition_function_harmonic(
                get_f(p, "omega")?,
                get_f(p, "temperature")?,
            ),
        )),
        "fermi_dirac_distribution" => Ok(RunOutput::Scalar(
            mathematical_physics::fermi_dirac_distribution(
                get_f(p, "energy")?,
                get_f(p, "chemical_potential")?,
                get_f(p, "temperature")?,
            ),
        )),
        "bose_einstein_distribution" => Ok(RunOutput::Scalar(
            mathematical_physics::bose_einstein_distribution(
                get_f(p, "energy")?,
                get_f(p, "chemical_potential")?,
                get_f(p, "temperature")?,
            ),
        )),
        "density_of_states_3d_free" => Ok(RunOutput::Scalar(
            mathematical_physics::density_of_states_3d_free(
                get_f(p, "energy")?,
                get_f(p, "volume")?,
                get_f(p, "mass")?,
            ),
        )),
        "fourier_mode_frequency" => Ok(RunOutput::Scalar(
            mathematical_physics::fourier_mode_frequency(
                get_f(p, "mode_number")?,
                get_f(p, "length")?,
                get_f(p, "wave_speed")?,
            ),
        )),
        "relativistic_energy" => Ok(RunOutput::Scalar(
            mathematical_physics::relativistic_energy(
                get_f(p, "rest_mass")?,
                get_f(p, "momentum")?,
            ),
        )),
        "thermal_wavelength" => Ok(RunOutput::Scalar(mathematical_physics::thermal_wavelength(
            get_f(p, "mass")?,
            get_f(p, "temperature")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "mathematical_physics: unknown function: {func}"
        ))),
    }
}
