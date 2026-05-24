//! Dispatch handler for colloid chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "stokes_sedimentation" => Ok(RunOutput::Scalar(
            chem::colloids::properties::stokes_sedimentation(
                get_f(p, "r")?,
                get_f(p, "rho_p")?,
                get_f(p, "rho_f")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "brownian_diffusion_coefficient" => Ok(RunOutput::Scalar(
            chem::colloids::properties::brownian_diffusion_coefficient(
                get_f(p, "t")?,
                get_f(p, "viscosity")?,
                get_f(p, "r")?,
            ),
        )),
        "einstein_diffusion_displacement" => Ok(RunOutput::Scalar(
            chem::colloids::properties::einstein_diffusion_displacement(
                get_f(p, "d")?,
                get_f(p, "t")?,
            ),
        )),
        "peclet_number_colloid" => Ok(RunOutput::Scalar(
            chem::colloids::properties::peclet_number_colloid(
                get_f(p, "velocity")?,
                get_f(p, "r")?,
                get_f(p, "d")?,
            ),
        )),
        "osmotic_pressure_colloid" => Ok(RunOutput::Scalar(
            chem::colloids::properties::osmotic_pressure_colloid(
                get_f(p, "n_particles")?,
                get_f(p, "volume")?,
                get_f(p, "t")?,
            ),
        )),
        "sedimentation_coefficient" => Ok(RunOutput::Scalar(
            chem::colloids::properties::sedimentation_coefficient(
                get_f(p, "velocity")?,
                get_f(p, "omega")?,
                get_f(p, "r_centrifuge")?,
            ),
        )),
        "tyndall_scattering_intensity" => Ok(RunOutput::Scalar(
            chem::colloids::properties::tyndall_scattering_intensity(
                get_f(p, "n")?,
                get_f(p, "v_particle")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "specific_surface_area" => Ok(RunOutput::Scalar(
            chem::colloids::properties::specific_surface_area(
                get_f(p, "radius")?,
                get_f(p, "density")?,
            ),
        )),
        "flocculation_rate_smoluchowski" => Ok(RunOutput::Scalar(
            chem::colloids::properties::flocculation_rate_smoluchowski(
                get_f(p, "n0")?,
                get_f(p, "k_b")?,
                get_f(p, "t")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "half_life_coagulation" => Ok(RunOutput::Scalar(
            chem::colloids::properties::half_life_coagulation(get_f(p, "n0")?, get_f(p, "k_rate")?),
        )),

        "dlvo_total_energy" => Ok(RunOutput::Scalar(
            chem::colloids::stability::dlvo_total_energy(
                get_f(p, "van_der_waals")?,
                get_f(p, "electrostatic")?,
            ),
        )),
        "hamaker_sphere_sphere" => Ok(RunOutput::Scalar(
            chem::colloids::stability::hamaker_sphere_sphere(
                get_f(p, "a_h")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "d")?,
            ),
        )),
        "hamaker_sphere_surface" => Ok(RunOutput::Scalar(
            chem::colloids::stability::hamaker_sphere_surface(
                get_f(p, "a_h")?,
                get_f(p, "r")?,
                get_f(p, "d")?,
            ),
        )),
        "debye_length" => Ok(RunOutput::Scalar(chem::colloids::stability::debye_length(
            get_f(p, "epsilon_r")?,
            get_f(p, "t")?,
            get_f(p, "ionic_strength")?,
        ))),
        "electrostatic_repulsion" => Ok(RunOutput::Scalar(
            chem::colloids::stability::electrostatic_repulsion(
                get_f(p, "epsilon_r")?,
                get_f(p, "r")?,
                get_f(p, "psi0")?,
                get_f(p, "kappa")?,
                get_f(p, "d")?,
            ),
        )),
        "zeta_potential_smoluchowski" => Ok(RunOutput::Scalar(
            chem::colloids::stability::zeta_potential_smoluchowski(
                get_f(p, "mobility")?,
                get_f(p, "viscosity")?,
                get_f(p, "epsilon")?,
            ),
        )),
        "schulze_hardy_ccc" => Ok(RunOutput::Scalar(
            chem::colloids::stability::schulze_hardy_ccc(get_i(p, "z")? as i32),
        )),
        "critical_coagulation_concentration" => Ok(RunOutput::Scalar(
            chem::colloids::stability::critical_coagulation_concentration(
                get_f(p, "epsilon")?,
                get_f(p, "t")?,
                get_f(p, "psi0")?,
                get_f(p, "a_h")?,
                get_f(p, "z")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
