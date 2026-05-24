//! Dispatch handler for stellar physics functions.

use super::super::params::*;
use crate::domain::astronomy as astro;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "luminosity_from_radius_temp" => Ok(RunOutput::Scalar(
            astro::stellar::luminosity_from_radius_temp(get_f(p, "r")?, get_f(p, "t")?),
        )),
        "absolute_magnitude" => Ok(RunOutput::Scalar(astro::stellar::absolute_magnitude(
            get_f(p, "apparent_mag")?,
            get_f(p, "distance_pc")?,
        ))),
        "distance_modulus" => Ok(RunOutput::Scalar(astro::stellar::distance_modulus(
            get_f(p, "apparent_mag")?,
            get_f(p, "absolute_mag")?,
        ))),
        "stellar_flux" => Ok(RunOutput::Scalar(astro::stellar::stellar_flux(
            get_f(p, "luminosity")?,
            get_f(p, "distance")?,
        ))),
        "wien_peak_wavelength" => Ok(RunOutput::Scalar(astro::stellar::wien_peak_wavelength(
            get_f(p, "temperature")?,
        ))),
        "main_sequence_lifetime" => Ok(RunOutput::Scalar(astro::stellar::main_sequence_lifetime(
            get_f(p, "mass_solar")?,
            get_f(p, "luminosity_solar")?,
        ))),
        "mass_luminosity_relation" => Ok(RunOutput::Scalar(
            astro::stellar::mass_luminosity_relation(get_f(p, "mass_solar")?),
        )),
        "schwarzschild_radius" => Ok(RunOutput::Scalar(astro::stellar::schwarzschild_radius(
            get_f(p, "mass")?,
        ))),
        "chandrasekhar_limit" => Ok(RunOutput::Scalar(astro::stellar::chandrasekhar_limit())),
        "eddington_luminosity" => Ok(RunOutput::Scalar(astro::stellar::eddington_luminosity(
            get_f(p, "mass")?,
        ))),
        "spectral_type_temperature" => Ok(RunOutput::Scalar(
            astro::stellar::spectral_type_temperature(get_f(p, "spectral_index")?),
        )),
        "bolometric_correction" => Ok(RunOutput::Scalar(astro::stellar::bolometric_correction(
            get_f(p, "t_eff")?,
        ))),
        "tov_limit" => Ok(RunOutput::Scalar(astro::stellar::tov_limit())),
        "neutron_star_surface_gravity" => Ok(RunOutput::Scalar(
            astro::stellar::neutron_star_surface_gravity(),
        )),
        "neutron_star_mean_density" => Ok(RunOutput::Scalar(
            astro::stellar::neutron_star_mean_density(),
        )),
        "neutron_star_escape_velocity" => Ok(RunOutput::Scalar(
            astro::stellar::neutron_star_escape_velocity(),
        )),
        "pulsar_spindown_luminosity" => Ok(RunOutput::Scalar(
            astro::stellar::pulsar_spindown_luminosity(
                get_f(p, "period")?,
                get_f(p, "period_dot")?,
            ),
        )),
        "pulsar_magnetic_field" => Ok(RunOutput::Scalar(astro::stellar::pulsar_magnetic_field(
            get_f(p, "period")?,
            get_f(p, "period_dot")?,
        ))),
        "pulsar_characteristic_age" => Ok(RunOutput::Scalar(
            astro::stellar::pulsar_characteristic_age(get_f(p, "period")?, get_f(p, "period_dot")?),
        )),
        "magnetar_energy_reservoir" => Ok(RunOutput::Scalar(
            astro::stellar::magnetar_energy_reservoir(get_f(p, "b_field")?),
        )),
        "magnetar_typical_energy" => {
            Ok(RunOutput::Scalar(astro::stellar::magnetar_typical_energy()))
        }
        "radiation_pressure" => Ok(RunOutput::Scalar(astro::stellar::radiation_pressure(
            get_f(p, "temperature")?,
        ))),
        "gas_pressure" => Ok(RunOutput::Scalar(astro::stellar::gas_pressure(
            get_f(p, "rho")?,
            get_f(p, "temperature")?,
            get_f(p, "mu")?,
        ))),
        "adiabatic_sound_speed" => Ok(RunOutput::Scalar(astro::stellar::adiabatic_sound_speed(
            get_f(p, "temperature")?,
            get_f(p, "mu")?,
        ))),
        "pp_chain_luminosity" => Ok(RunOutput::Scalar(astro::stellar::pp_chain_luminosity(
            get_f(p, "mass_kg")?,
            get_f(p, "x_hydrogen")?,
        ))),
        "cno_cycle_luminosity" => Ok(RunOutput::Scalar(astro::stellar::cno_cycle_luminosity(
            get_f(p, "mass_kg")?,
            get_f(p, "x_hydrogen")?,
            get_f(p, "z_metals")?,
        ))),
        "kelvin_helmholtz_timescale" => Ok(RunOutput::Scalar(
            astro::stellar::kelvin_helmholtz_timescale(
                get_f(p, "mass")?,
                get_f(p, "radius")?,
                get_f(p, "luminosity")?,
            ),
        )),
        "nuclear_timescale" => Ok(RunOutput::Scalar(astro::stellar::nuclear_timescale(
            get_f(p, "mass")?,
            get_f(p, "luminosity")?,
        ))),
        "white_dwarf_radius_from_mass" => Ok(RunOutput::Scalar(
            astro::stellar::white_dwarf_radius_from_mass(get_f(p, "mass_kg")?),
        )),
        "eddington_luminosity_numerical" => Ok(RunOutput::Scalar(
            astro::stellar::eddington_luminosity_numerical(get_f(p, "mass_solar")?),
        )),
        "solar_effective_temperature" => Ok(RunOutput::Scalar(
            astro::stellar::solar_effective_temperature(),
        )),
        "solar_absolute_magnitude" => {
            Ok(RunOutput::Scalar(astro::stellar::solar_absolute_magnitude()))
        }
        "solar_metallicity" => Ok(RunOutput::Scalar(astro::stellar::solar_metallicity())),
        "solar_main_sequence_lifetime" => Ok(RunOutput::Scalar(
            astro::stellar::solar_main_sequence_lifetime(),
        )),
        "hydrogen_burning_energy_per_kg" => Ok(RunOutput::Scalar(
            astro::stellar::hydrogen_burning_energy_per_kg(),
        )),
        "eddington_ratio" => Ok(RunOutput::Scalar(astro::stellar::eddington_ratio(
            get_f(p, "luminosity")?,
            get_f(p, "mass")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
