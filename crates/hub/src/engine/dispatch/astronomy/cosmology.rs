//! Dispatch handler for cosmology functions.

use super::super::params::*;
use crate::domain::astronomy as astro;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "hubble_velocity" => Ok(RunOutput::Scalar(astro::cosmology::hubble_velocity(
            get_f(p, "h0")?,
            get_f(p, "distance")?,
        ))),
        "hubble_distance" => Ok(RunOutput::Scalar(astro::cosmology::hubble_distance(
            get_f(p, "h0")?,
            get_f(p, "velocity")?,
        ))),
        "redshift_velocity" => Ok(RunOutput::Scalar(astro::cosmology::redshift_velocity(
            get_f(p, "v")?,
            get_f(p, "c")?,
        ))),
        "cosmological_redshift" => Ok(RunOutput::Scalar(astro::cosmology::cosmological_redshift(
            get_f(p, "a_emit")?,
            get_f(p, "a_obs")?,
        ))),
        "scale_factor" => Ok(RunOutput::Scalar(astro::cosmology::scale_factor(get_f(
            p, "redshift",
        )?))),
        "critical_density" => Ok(RunOutput::Scalar(astro::cosmology::critical_density(
            get_f(p, "h")?,
        ))),
        "friedmann_expansion" => Ok(RunOutput::Scalar(astro::cosmology::friedmann_expansion(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "a")?,
        ))),
        "lookback_time" => Ok(RunOutput::Scalar(astro::cosmology::lookback_time(
            get_f(p, "z")?,
            get_f(p, "h0")?,
        ))),
        "luminosity_distance" => Ok(RunOutput::Scalar(astro::cosmology::luminosity_distance(
            get_f(p, "comoving_distance")?,
            get_f(p, "z")?,
        ))),
        "angular_diameter_distance" => Ok(RunOutput::Scalar(
            astro::cosmology::angular_diameter_distance(
                get_f(p, "comoving_distance")?,
                get_f(p, "z")?,
            ),
        )),
        "cmb_temperature" => Ok(RunOutput::Scalar(astro::cosmology::cmb_temperature(
            get_f(p, "t0")?,
            get_f(p, "z")?,
        ))),
        "age_of_universe" => Ok(RunOutput::Scalar(astro::cosmology::age_of_universe(get_f(
            p, "h0",
        )?))),
        "e_z" => Ok(RunOutput::Scalar(astro::cosmology::e_z(
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_k")?,
            get_f(p, "omega_de")?,
            get_f(p, "z")?,
        ))),
        "e_z_lcdm" => Ok(RunOutput::Scalar(astro::cosmology::e_z_lcdm(
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "e_z_wcdm" => Ok(RunOutput::Scalar(astro::cosmology::e_z_wcdm(
            get_f(p, "omega_m")?,
            get_f(p, "omega_de")?,
            get_f(p, "w")?,
            get_f(p, "z")?,
        ))),
        "e_z_w0wa" => Ok(RunOutput::Scalar(astro::cosmology::e_z_w0wa(
            get_f(p, "omega_m")?,
            get_f(p, "omega_de")?,
            get_f(p, "w0")?,
            get_f(p, "wa")?,
            get_f(p, "z")?,
        ))),
        "hubble_at_z" => Ok(RunOutput::Scalar(astro::cosmology::hubble_at_z(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_k")?,
            get_f(p, "omega_de")?,
            get_f(p, "z")?,
        ))),
        "hubble_distance_mpc" => Ok(RunOutput::Scalar(astro::cosmology::hubble_distance_mpc(
            get_f(p, "h0")?,
        ))),
        "hubble_time_gyr" => Ok(RunOutput::Scalar(astro::cosmology::hubble_time_gyr(get_f(
            p, "h0",
        )?))),
        "omega_k_from" => Ok(RunOutput::Scalar(astro::cosmology::omega_k_from(
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_de")?,
        ))),
        "comoving_distance_from_z" => Ok(RunOutput::Scalar(
            astro::cosmology::comoving_distance_from_z(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "transverse_comoving_distance" => Ok(RunOutput::Scalar(
            astro::cosmology::transverse_comoving_distance(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
                get_f(p, "z")?,
            ),
        )),
        "luminosity_distance_from_z" => Ok(RunOutput::Scalar(
            astro::cosmology::luminosity_distance_from_z(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "angular_diameter_distance_from_z" => Ok(RunOutput::Scalar(
            astro::cosmology::angular_diameter_distance_from_z(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "distance_modulus_from_z" => Ok(RunOutput::Scalar(
            astro::cosmology::distance_modulus_from_z(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "lookback_time_from_z" => Ok(RunOutput::Scalar(astro::cosmology::lookback_time_from_z(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "age_lcdm" => Ok(RunOutput::Scalar(astro::cosmology::age_lcdm(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
        ))),
        "comoving_volume_element" => Ok(RunOutput::Scalar(
            astro::cosmology::comoving_volume_element(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "comoving_volume_total" => Ok(RunOutput::Scalar(astro::cosmology::comoving_volume_total(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "comoving_volume_shell" => Ok(RunOutput::Scalar(astro::cosmology::comoving_volume_shell(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z1")?,
            get_f(p, "z2")?,
        ))),
        "critical_density_from_h0" => Ok(RunOutput::Scalar(
            astro::cosmology::critical_density_from_h0(get_f(p, "h0")?),
        )),
        "relativistic_redshift" => Ok(RunOutput::Scalar(astro::cosmology::relativistic_redshift(
            get_f(p, "v")?,
        ))),
        "velocity_from_redshift" => Ok(RunOutput::Scalar(
            astro::cosmology::velocity_from_redshift(get_f(p, "z")?),
        )),
        "e_z_lcdm_rad" => Ok(RunOutput::Scalar(astro::cosmology::e_z_lcdm_rad(
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "z")?,
        ))),
        "hubble_at_z_lcdm" => Ok(RunOutput::Scalar(astro::cosmology::hubble_at_z_lcdm(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "deceleration_parameter" => Ok(RunOutput::Scalar(
            astro::cosmology::deceleration_parameter(get_f(p, "omega_m")?, get_f(p, "z")?),
        )),
        "deceleration_parameter_today" => Ok(RunOutput::Scalar(
            astro::cosmology::deceleration_parameter_today(get_f(p, "omega_m")?),
        )),
        "little_h" => Ok(RunOutput::Scalar(astro::cosmology::little_h(get_f(
            p, "h0",
        )?))),
        "comoving_distance_general" => Ok(RunOutput::Scalar(
            astro::cosmology::comoving_distance_general(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
                get_f(p, "z")?,
            ),
        )),
        "luminosity_distance_general" => Ok(RunOutput::Scalar(
            astro::cosmology::luminosity_distance_general(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
                get_f(p, "z")?,
            ),
        )),
        "angular_diameter_distance_general" => Ok(RunOutput::Scalar(
            astro::cosmology::angular_diameter_distance_general(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
                get_f(p, "z")?,
            ),
        )),
        "distance_modulus_general" => Ok(RunOutput::Scalar(
            astro::cosmology::distance_modulus_general(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
                get_f(p, "z")?,
            ),
        )),
        "comoving_distance_wcdm" => {
            Ok(RunOutput::Scalar(astro::cosmology::comoving_distance_wcdm(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_de")?,
                get_f(p, "w")?,
                get_f(p, "z")?,
            )))
        }
        "luminosity_distance_wcdm" => Ok(RunOutput::Scalar(
            astro::cosmology::luminosity_distance_wcdm(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_de")?,
                get_f(p, "w")?,
                get_f(p, "z")?,
            ),
        )),
        "comoving_distance_w0wa" => {
            Ok(RunOutput::Scalar(astro::cosmology::comoving_distance_w0wa(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_de")?,
                get_f(p, "w0")?,
                get_f(p, "wa")?,
                get_f(p, "z")?,
            )))
        }
        "luminosity_distance_w0wa" => Ok(RunOutput::Scalar(
            astro::cosmology::luminosity_distance_w0wa(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_de")?,
                get_f(p, "w0")?,
                get_f(p, "wa")?,
                get_f(p, "z")?,
            ),
        )),
        "luminosity_from_angular_diameter" => Ok(RunOutput::Scalar(
            astro::cosmology::luminosity_from_angular_diameter(get_f(p, "d_a")?, get_f(p, "z")?),
        )),
        "angular_diameter_from_luminosity" => Ok(RunOutput::Scalar(
            astro::cosmology::angular_diameter_from_luminosity(get_f(p, "d_l")?, get_f(p, "z")?),
        )),
        "proper_distance" => Ok(RunOutput::Scalar(astro::cosmology::proper_distance(
            get_f(p, "comoving_d")?,
            get_f(p, "z")?,
        ))),
        "lookback_time_general" => Ok(RunOutput::Scalar(astro::cosmology::lookback_time_general(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_k")?,
            get_f(p, "omega_de")?,
            get_f(p, "z")?,
        ))),
        "age_at_z" => Ok(RunOutput::Scalar(astro::cosmology::age_at_z(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "age_general" => Ok(RunOutput::Scalar(astro::cosmology::age_general(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_r")?,
            get_f(p, "omega_k")?,
            get_f(p, "omega_de")?,
        ))),
        "acceleration_redshift" => Ok(RunOutput::Scalar(astro::cosmology::acceleration_redshift(
            get_f(p, "omega_m")?,
        ))),
        "matter_radiation_equality_z" => Ok(RunOutput::Scalar(
            astro::cosmology::matter_radiation_equality_z(
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
            ),
        )),
        "particle_horizon" => Ok(RunOutput::Scalar(astro::cosmology::particle_horizon(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
        ))),
        "particle_horizon_general" => Ok(RunOutput::Scalar(
            astro::cosmology::particle_horizon_general(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
                get_f(p, "omega_k")?,
                get_f(p, "omega_de")?,
            ),
        )),
        "sound_horizon_eh98" => Ok(RunOutput::Scalar(astro::cosmology::sound_horizon_eh98(
            get_f(p, "omega_m")?,
            get_f(p, "omega_b")?,
            get_f(p, "h0")?,
        ))),
        "event_horizon" => Ok(RunOutput::Scalar(astro::cosmology::event_horizon(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
        ))),
        "matter_density_at_z" => Ok(RunOutput::Scalar(astro::cosmology::matter_density_at_z(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "radiation_density_at_z" => {
            Ok(RunOutput::Scalar(astro::cosmology::radiation_density_at_z(
                get_f(p, "h0")?,
                get_f(p, "omega_r")?,
                get_f(p, "z")?,
            )))
        }
        "dark_energy_density_at_z" => Ok(RunOutput::Scalar(
            astro::cosmology::dark_energy_density_at_z(
                get_f(p, "h0")?,
                get_f(p, "omega_de")?,
                get_f(p, "w")?,
                get_f(p, "z")?,
            ),
        )),
        "omega_m_at_z" => Ok(RunOutput::Scalar(astro::cosmology::omega_m_at_z(
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "omega_de_at_z" => Ok(RunOutput::Scalar(astro::cosmology::omega_de_at_z(
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "growth_factor_approx" => Ok(RunOutput::Scalar(astro::cosmology::growth_factor_approx(
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "growth_rate" => Ok(RunOutput::Scalar(astro::cosmology::growth_rate(
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "sigma8_at_z" => Ok(RunOutput::Scalar(astro::cosmology::sigma8_at_z(
            get_f(p, "sigma8")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "cmb_temperature_at_z" => Ok(RunOutput::Scalar(astro::cosmology::cmb_temperature_at_z(
            get_f(p, "z")?,
        ))),
        "photon_number_density" => Ok(RunOutput::Scalar(astro::cosmology::photon_number_density(
            get_f(p, "z")?,
        ))),
        "cmb_energy_density" => Ok(RunOutput::Scalar(astro::cosmology::cmb_energy_density(
            get_f(p, "z")?,
        ))),
        "omega_r_from_tcmb" => Ok(RunOutput::Scalar(astro::cosmology::omega_r_from_tcmb(
            get_f(p, "t_cmb")?,
            get_f(p, "h0")?,
        ))),
        "einstein_radius" => Ok(RunOutput::Scalar(astro::cosmology::einstein_radius(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "mass")?,
            get_f(p, "z_l")?,
            get_f(p, "z_s")?,
        ))),
        "critical_surface_density" => Ok(RunOutput::Scalar(
            astro::cosmology::critical_surface_density(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z_l")?,
                get_f(p, "z_s")?,
            ),
        )),
        "sn1a_apparent_magnitude" => Ok(RunOutput::Scalar(
            astro::cosmology::sn1a_apparent_magnitude(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
                get_f(p, "abs_mag")?,
            ),
        )),
        "sn1a_chi2_single" => Ok(RunOutput::Scalar(astro::cosmology::sn1a_chi2_single(
            get_f(p, "m_obs")?,
            get_f(p, "m_model")?,
            get_f(p, "sigma")?,
        ))),
        "dv_bao" => Ok(RunOutput::Scalar(astro::cosmology::dv_bao(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "hubble_distance_at_z" => Ok(RunOutput::Scalar(astro::cosmology::hubble_distance_at_z(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "rd_over_dv" => Ok(RunOutput::Scalar(astro::cosmology::rd_over_dv(
            get_f(p, "r_d")?,
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "virial_radius" => Ok(RunOutput::Scalar(astro::cosmology::virial_radius(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "mass_solar")?,
            get_f(p, "z")?,
        ))),
        "nfw_concentration_duffy08" => Ok(RunOutput::Scalar(
            astro::cosmology::nfw_concentration_duffy08(get_f(p, "mass_solar")?, get_f(p, "z")?),
        )),
        "virial_velocity" => Ok(RunOutput::Scalar(astro::cosmology::virial_velocity(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "mass_solar")?,
            get_f(p, "z")?,
        ))),
        "photon_density_today" => Ok(RunOutput::Scalar(astro::cosmology::photon_density_today())),
        "baryon_to_photon_ratio" => Ok(RunOutput::Scalar(
            astro::cosmology::baryon_to_photon_ratio(get_f(p, "omega_b")?, get_f(p, "h0")?),
        )),
        "neutrino_temperature" => Ok(RunOutput::Scalar(astro::cosmology::neutrino_temperature())),
        "n_eff_standard" => Ok(RunOutput::Scalar(astro::cosmology::n_eff_standard())),
        "omega_r_with_neutrinos" => Ok(RunOutput::Scalar(
            astro::cosmology::omega_r_with_neutrinos(get_f(p, "omega_gamma")?, get_f(p, "n_eff")?),
        )),
        "jeans_length" => Ok(RunOutput::Scalar(astro::cosmology::jeans_length(
            get_f(p, "sound_speed")?,
            get_f(p, "density")?,
        ))),
        "jeans_mass" => Ok(RunOutput::Scalar(astro::cosmology::jeans_mass(
            get_f(p, "sound_speed")?,
            get_f(p, "density")?,
        ))),
        "redshift_from_temperature" => Ok(RunOutput::Scalar(
            astro::cosmology::redshift_from_temperature(get_f(p, "t")?),
        )),
        "conformal_time" => Ok(RunOutput::Scalar(astro::cosmology::conformal_time(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "conformal_distance" => Ok(RunOutput::Scalar(astro::cosmology::conformal_distance(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "nfw_density_profile" => Ok(RunOutput::Scalar(astro::cosmology::nfw_density_profile(
            get_f(p, "r_kpc")?,
        ))),
        "nfw_enclosed_mass" => Ok(RunOutput::Scalar(astro::cosmology::nfw_enclosed_mass(
            get_f(p, "r_kpc")?,
        ))),
        "nfw_circular_velocity" => Ok(RunOutput::Scalar(astro::cosmology::nfw_circular_velocity(
            get_f(p, "r_kpc")?,
        ))),
        "nfw_concentration" => Ok(RunOutput::Scalar(astro::cosmology::nfw_concentration())),
        "nfw_virial_mass" => Ok(RunOutput::Scalar(astro::cosmology::nfw_virial_mass())),
        "nfw_virial_radius" => Ok(RunOutput::Scalar(astro::cosmology::nfw_virial_radius())),
        "wimp_annihilation_rate" => Ok(RunOutput::Scalar(
            astro::cosmology::wimp_annihilation_rate(get_f(p, "rho")?, get_f(p, "mass_gev")?),
        )),
        "wimp_direct_detection_rate" => Ok(RunOutput::Scalar(
            astro::cosmology::wimp_direct_detection_rate(
                get_f(p, "rho")?,
                get_f(p, "mass_gev")?,
                get_f(p, "sigma_cm2")?,
                get_f(p, "v_mean")?,
            ),
        )),
        "local_dm_density_gev" => Ok(RunOutput::Scalar(astro::cosmology::local_dm_density_gev())),
        "local_dm_density_si" => Ok(RunOutput::Scalar(astro::cosmology::local_dm_density_si())),
        "thermal_relic_cross_section" => Ok(RunOutput::Scalar(
            astro::cosmology::thermal_relic_cross_section(),
        )),
        "axion_compton_frequency" => Ok(RunOutput::Scalar(
            astro::cosmology::axion_compton_frequency(get_f(p, "mass_ev")?),
        )),
        "axion_de_broglie_wavelength" => Ok(RunOutput::Scalar(
            astro::cosmology::axion_de_broglie_wavelength(
                get_f(p, "mass_ev")?,
                get_f(p, "velocity")?,
            ),
        )),
        "dm_relic_density" => Ok(RunOutput::Scalar(astro::cosmology::dm_relic_density())),
        "baryon_density" => Ok(RunOutput::Scalar(astro::cosmology::baryon_density())),
        "matter_density" => Ok(RunOutput::Scalar(astro::cosmology::matter_density())),
        "dark_energy_density_today" => Ok(RunOutput::Scalar(
            astro::cosmology::dark_energy_density_today(),
        )),
        "matter_radiation_equality_temperature" => Ok(RunOutput::Scalar(
            astro::cosmology::matter_radiation_equality_temperature(),
        )),
        "matter_radiation_equality_redshift" => Ok(RunOutput::Scalar(
            astro::cosmology::matter_radiation_equality_redshift(),
        )),
        "recombination_temperature" => Ok(RunOutput::Scalar(
            astro::cosmology::recombination_temperature(),
        )),
        "recombination_redshift" => {
            Ok(RunOutput::Scalar(astro::cosmology::recombination_redshift()))
        }
        "age_of_universe_seconds" => Ok(RunOutput::Scalar(
            astro::cosmology::age_of_universe_seconds(),
        )),
        "baryon_to_dm_ratio" => Ok(RunOutput::Scalar(astro::cosmology::baryon_to_dm_ratio())),
        "dm_annihilation_luminosity" => Ok(RunOutput::Scalar(
            astro::cosmology::dm_annihilation_luminosity(
                get_f(p, "rho")?,
                get_f(p, "mass_gev")?,
                get_f(p, "volume")?,
            ),
        )),
        "saha_ionization_fraction" => Ok(RunOutput::Scalar(
            astro::cosmology::saha_ionization_fraction(get_f(p, "temperature")?, get_f(p, "n_b")?),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
