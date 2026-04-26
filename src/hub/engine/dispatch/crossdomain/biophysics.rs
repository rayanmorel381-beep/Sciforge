//! Dispatch handler for biophysics functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::biophysics;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches a biophysics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "diffusion_coefficient_stokes_einstein" => Ok(RunOutput::Scalar(
            biophysics::diffusion_coefficient_stokes_einstein(
                get_f(p, "temperature")?,
                get_f(p, "viscosity")?,
                get_f(p, "radius")?,
            ),
        )),
        "membrane_capacitance" => Ok(RunOutput::Scalar(biophysics::membrane_capacitance(
            get_f(p, "area")?,
            get_f(p, "thickness")?,
            get_f(p, "dielectric_constant")?,
        ))),
        "stokes_drag_force" => Ok(RunOutput::Scalar(biophysics::stokes_drag_force(
            get_f(p, "viscosity")?,
            get_f(p, "radius")?,
            get_f(p, "velocity")?,
        ))),
        "sedimentation_coefficient" => {
            Ok(RunOutput::Scalar(biophysics::sedimentation_coefficient(
                get_f(p, "particle_mass")?,
                get_f(p, "solvent_density")?,
                get_f(p, "particle_density")?,
                get_f(p, "friction_coefficient")?,
            )))
        }
        "thermal_fluctuation_amplitude" => Ok(RunOutput::Scalar(
            biophysics::thermal_fluctuation_amplitude(
                get_f(p, "temperature")?,
                get_f(p, "spring_constant")?,
            ),
        )),
        "worm_like_chain_extension" => {
            Ok(RunOutput::Scalar(biophysics::worm_like_chain_extension(
                get_f(p, "force")?,
                get_f(p, "contour_length")?,
                get_f(p, "persistence_length")?,
                get_f(p, "temperature")?,
            )))
        }
        "reynolds_number" => Ok(RunOutput::Scalar(biophysics::reynolds_number(
            get_f(p, "density")?,
            get_f(p, "velocity")?,
            get_f(p, "length")?,
            get_f(p, "viscosity")?,
        ))),
        "fick_diffusion_flux" => Ok(RunOutput::Scalar(biophysics::fick_diffusion_flux(
            get_f(p, "diffusion_coeff")?,
            get_f(p, "concentration_gradient")?,
        ))),
        "debye_screening_length" => Ok(RunOutput::Scalar(biophysics::debye_screening_length(
            get_f(p, "temperature")?,
            get_f(p, "ionic_strength")?,
            get_f(p, "dielectric_constant")?,
        ))),
        "electrophoretic_mobility" => Ok(RunOutput::Scalar(biophysics::electrophoretic_mobility(
            get_f(p, "charge")?,
            get_f(p, "friction_coefficient")?,
        ))),
        "helfrich_bending_energy" => Ok(RunOutput::Scalar(biophysics::helfrich_bending_energy(
            get_f(p, "bending_modulus")?,
            get_f(p, "mean_curvature")?,
            get_f(p, "spontaneous_curvature")?,
            get_f(p, "area")?,
        ))),
        _ => Err(HubError::InvalidInput(format!(
            "biophysics: unknown function: {func}"
        ))),
    }
}
