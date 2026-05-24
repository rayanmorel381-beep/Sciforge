//! Dispatch handler for planetary geology functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::cross_domain::planetary_geology;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;

/// Dispatches a planetary geology function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "impact_energy" => Ok(RunOutput::Scalar(planetary_geology::impact_energy(
            get_f(p, "projectile_mass")?,
            get_f(p, "impact_velocity")?,
        ))),
        "crater_scaling_diameter" => Ok(RunOutput::Scalar(
            planetary_geology::crater_scaling_diameter(
                get_f(p, "projectile_diameter")?,
                get_f(p, "projectile_density")?,
                get_f(p, "target_density")?,
                get_f(p, "velocity")?,
                get_f(p, "gravity")?,
            ),
        )),
        "tidal_heating_rate" => Ok(RunOutput::Scalar(planetary_geology::tidal_heating_rate(
            get_f(p, "eccentricity")?,
            get_f(p, "mean_motion")?,
            get_f(p, "planet_radius")?,
            get_f(p, "quality_param")?,
            get_f(p, "primary_mass")?,
            get_f(p, "semi_major")?,
        ))),
        "surface_temperature_equilibrium" => Ok(RunOutput::Scalar(
            planetary_geology::surface_temperature_equilibrium(
                get_f(p, "solar_flux")?,
                get_f(p, "albedo")?,
                get_f(p, "emissivity")?,
            ),
        )),
        "lava_flow_cooling_time" => Ok(RunOutput::Scalar(
            planetary_geology::lava_flow_cooling_time(
                get_f(p, "thickness")?,
                get_f(p, "thermal_diffusivity")?,
            ),
        )),
        "regolith_depth" => Ok(RunOutput::Scalar(planetary_geology::regolith_depth(
            get_f(p, "flux_rate")?,
            get_f(p, "surface_density")?,
            get_f(p, "time")?,
        ))),
        "lithospheric_thickness" => Ok(RunOutput::Scalar(
            planetary_geology::lithospheric_thickness(
                get_f(p, "thermal_conductivity")?,
                get_f(p, "heat_flow")?,
                get_f(p, "base_temp")?,
                get_f(p, "surface_temp")?,
            ),
        )),
        "gravitational_differentiation_time" => Ok(RunOutput::Scalar(
            planetary_geology::gravitational_differentiation_time(
                get_f(p, "radius")?,
                get_f(p, "density_diff")?,
                get_f(p, "gravity")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "crater_counting_surface_age" => Ok(RunOutput::Scalar(
            planetary_geology::crater_counting_surface_age(
                get_f(p, "crater_density")?,
                get_f(p, "production_rate")?,
            ),
        )),
        "volcanic_effusion_rate" => Ok(RunOutput::Scalar(
            planetary_geology::volcanic_effusion_rate(
                get_f(p, "thermal_flux")?,
                get_f(p, "specific_heat")?,
                get_f(p, "delta_t")?,
                get_f(p, "latent_heat")?,
            ),
        )),
        "ejecta_blanket_thickness" => Ok(RunOutput::Scalar(
            planetary_geology::ejecta_blanket_thickness(
                get_f(p, "crater_radius")?,
                get_f(p, "distance_from_center")?,
                get_f(p, "rim_thickness")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!(
            "planetary_geology: unknown function: {func}"
        ))),
    }
}
