//! Dispatch handler for geophysics functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::cross_domain::geophysics;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;

/// Dispatches a geophysics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "radiogenic_heat_production" => {
            Ok(RunOutput::Scalar(geophysics::radiogenic_heat_production(
                get_f(p, "c_u238")?,
                get_f(p, "c_th232")?,
                get_f(p, "c_k40")?,
                get_f(p, "density")?,
            )))
        }
        "bouguer_anomaly" => Ok(RunOutput::Scalar(geophysics::bouguer_anomaly(
            get_f(p, "observed_gravity")?,
            get_f(p, "reference_gravity")?,
            get_f(p, "elevation")?,
            get_f(p, "slab_density")?,
        ))),
        "gravity_anomaly_buried_sphere" => Ok(RunOutput::Scalar(
            geophysics::gravity_anomaly_buried_sphere(
                get_f(p, "delta_rho")?,
                get_f(p, "radius")?,
                get_f(p, "center_depth")?,
                get_f(p, "horizontal_distance")?,
            ),
        )),
        "magnetic_anomaly_vertical_dipole" => Ok(RunOutput::Scalar(
            geophysics::magnetic_anomaly_vertical_dipole(
                get_f(p, "moment")?,
                get_f(p, "depth")?,
                get_f(p, "horizontal_distance")?,
            ),
        )),
        "seismic_wave_attenuation" => Ok(RunOutput::Scalar(geophysics::seismic_wave_attenuation(
            get_f(p, "amplitude_0")?,
            get_f(p, "frequency")?,
            get_f(p, "travel_time")?,
            get_f(p, "quality_factor")?,
        ))),
        "seismic_impedance_reflection" => {
            Ok(RunOutput::Scalar(geophysics::seismic_impedance_reflection(
                get_f(p, "rho1")?,
                get_f(p, "v1")?,
                get_f(p, "rho2")?,
                get_f(p, "v2")?,
            )))
        }
        "lithospheric_flexure_wavelength" => Ok(RunOutput::Scalar(
            geophysics::lithospheric_flexure_wavelength(
                get_f(p, "flexural_rigidity")?,
                get_f(p, "mantle_density")?,
                get_f(p, "infill_density")?,
                get_f(p, "g_surface")?,
            ),
        )),
        "isostatic_rebound_timescale" => {
            Ok(RunOutput::Scalar(geophysics::isostatic_rebound_timescale(
                get_f(p, "viscosity")?,
                get_f(p, "mantle_density")?,
                get_f(p, "g_surface")?,
                get_f(p, "wavelength")?,
            )))
        }
        "curie_depth" => Ok(RunOutput::Scalar(geophysics::curie_depth(
            get_f(p, "surface_heat_flow")?,
            get_f(p, "thermal_conductivity")?,
            get_f(p, "curie_temperature")?,
            get_f(p, "surface_temperature")?,
        ))),
        "radiative_thermal_conductivity" => Ok(RunOutput::Scalar(
            geophysics::radiative_thermal_conductivity(
                get_f(p, "temperature")?,
                get_f(p, "refractive_index")?,
                get_f(p, "absorption_coefficient")?,
            ),
        )),
        "electromagnetic_skin_depth" => {
            Ok(RunOutput::Scalar(geophysics::electromagnetic_skin_depth(
                get_f(p, "resistivity")?,
                get_f(p, "frequency")?,
            )))
        }
        _ => Err(HubError::InvalidInput(format!(
            "geophysics: unknown function: {func}"
        ))),
    }
}
