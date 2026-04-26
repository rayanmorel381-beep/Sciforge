//! Dispatch handler for astrochemistry functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::astrochemistry;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches an astrochemistry function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "jeans_mass" => Ok(RunOutput::Scalar(astrochemistry::jeans_mass(
            get_f(p, "temperature")?,
            get_f(p, "number_density")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "jeans_length" => Ok(RunOutput::Scalar(astrochemistry::jeans_length(
            get_f(p, "temperature")?,
            get_f(p, "number_density")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "freefall_time" => Ok(RunOutput::Scalar(astrochemistry::freefall_time(
            get_f(p, "number_density")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "cloud_thermal_velocity" => Ok(RunOutput::Scalar(astrochemistry::cloud_thermal_velocity(
            get_f(p, "temperature")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "bonnor_ebert_mass" => Ok(RunOutput::Scalar(astrochemistry::bonnor_ebert_mass(
            get_f(p, "temperature")?,
            get_f(p, "external_pressure")?,
            get_f(p, "mean_molecular_weight")?,
        ))),
        "photodissociation_rate" => Ok(RunOutput::Scalar(astrochemistry::photodissociation_rate(
            get_f(p, "unshielded_rate")?,
            get_f(p, "uv_field_habing")?,
            get_f(p, "shielding_factor")?,
            get_f(p, "visual_extinction")?,
        ))),
        "thermal_desorption_rate" => {
            Ok(RunOutput::Scalar(astrochemistry::thermal_desorption_rate(
                get_f(p, "attempt_frequency")?,
                get_f(p, "binding_energy")?,
                get_f(p, "dust_temperature")?,
            )))
        }
        "h2_formation_rate_on_dust" => Ok(RunOutput::Scalar(
            astrochemistry::h2_formation_rate_on_dust(
                get_f(p, "sticking_coefficient")?,
                get_f(p, "grain_cross_section")?,
                get_f(p, "grain_density")?,
                get_f(p, "temperature")?,
            ),
        )),
        "saha_ionization_ratio" => Ok(RunOutput::Scalar(astrochemistry::saha_ionization_ratio(
            get_f(p, "temperature")?,
            get_f(p, "electron_density")?,
            get_f(p, "ionization_energy")?,
            get_f(p, "partition_ratio")?,
        ))),
        "stroemgren_radius" => Ok(RunOutput::Scalar(astrochemistry::stroemgren_radius(
            get_f(p, "ionizing_photon_rate")?,
            get_f(p, "hydrogen_density")?,
            get_f(p, "recombination_coeff")?,
        ))),
        "dust_equilibrium_temperature" => Ok(RunOutput::Scalar(
            astrochemistry::dust_equilibrium_temperature(
                get_f(p, "luminosity")?,
                get_f(p, "distance")?,
                get_f(p, "absorption_efficiency")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!(
            "astrochemistry: unknown function: {func}"
        ))),
    }
}
