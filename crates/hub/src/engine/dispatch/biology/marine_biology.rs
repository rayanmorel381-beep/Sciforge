//! Dispatch handler for marine biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bleaching_thermal_threshold" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::bleaching_thermal_threshold(
                get_f(p, "sst")?,
                get_f(p, "mmm")?,
            ),
        )),
        "dhw_accumulation" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::dhw_accumulation(
                get_f(p, "dhw_accumulated")?,
                get_f(p, "hotspot")?,
                get_f(p, "dt_weeks")?,
            ),
        )),
        "calcification_rate" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::calcification_rate(
                get_f(p, "omega_aragonite")?,
                get_f(p, "temperature")?,
                get_f(p, "light")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "reef_rugosity" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::reef_rugosity(
                get_f(p, "surface_distance")?,
                get_f(p, "linear_distance")?,
            ),
        )),
        "coral_recruitment_rate" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::coral_recruitment_rate(
                get_f(p, "larval_supply")?,
                get_f(p, "settlement_rate")?,
                get_f(p, "post_settlement_survival")?,
                get_f(p, "available_substrate")?,
            ),
        )),
        "symbiodinium_density" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::symbiodinium_density(
                get_f(p, "photosynthesis_rate")?,
                get_f(p, "respiration_rate")?,
                get_f(p, "expulsion_rate")?,
                get_f(p, "dt")?,
                get_f(p, "current_density")?,
            ),
        )),
        "ocean_acidification_saturation" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::ocean_acidification_saturation(
                get_f(p, "ca_conc")?,
                get_f(p, "co3_conc")?,
                get_f(p, "ksp")?,
            ),
        )),
        "mangrove_carbon_sequestration" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::mangrove_carbon_sequestration(
                get_f(p, "biomass")?,
                get_f(p, "carbon_fraction")?,
                get_f(p, "growth_rate")?,
            ),
        )),
        "seagrass_light_attenuation" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::seagrass_light_attenuation(
                get_f(p, "surface_irradiance")?,
                get_f(p, "attenuation_coeff")?,
                get_f(p, "depth")?,
            ),
        )),
        "marine_protected_area_spillover" => Ok(RunOutput::Scalar(
            bio::marine_biology::coral::marine_protected_area_spillover(
                get_f(p, "biomass_inside")?,
                get_f(p, "biomass_outside")?,
                get_f(p, "perimeter_area_ratio")?,
                get_f(p, "diffusion")?,
            ),
        )),
        "marine_trophic_transfer_efficiency" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::trophic_transfer_efficiency(
                get_f(p, "production_upper")?,
                get_f(p, "production_lower")?,
            ),
        )),
        "fish_growth_von_bertalanffy" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::fish_growth_von_bertalanffy(
                get_f(p, "l_inf")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
                get_f(p, "t0")?,
            ),
        )),
        "fish_mortality_total" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::fish_mortality_total(
                get_f(p, "natural")?,
                get_f(p, "fishing")?,
            ),
        )),
        "maximum_sustainable_yield" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::maximum_sustainable_yield(
                get_f(p, "r")?,
                get_f(p, "k")?,
            ),
        )),
        "stock_recruitment_beverton_holt" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::stock_recruitment_beverton_holt(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "spawners")?,
            ),
        )),
        "stock_recruitment_ricker" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::stock_recruitment_ricker(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "spawners")?,
            ),
        )),
        "catch_per_unit_effort" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::catch_per_unit_effort(
                get_f(p, "catch")?,
                get_f(p, "effort")?,
            ),
        )),
        "decompression_no_stop_time" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::decompression_no_stop_time(
                get_f(p, "depth_m")?,
                get_f(p, "surface_rate")?,
            ),
        )),
        "schaefer_biomass" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::schaefer_biomass(
                get_f(p, "biomass")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "catch")?,
                get_f(p, "dt")?,
            ),
        )),
        "fishing_mortality_from_effort" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::fishing_mortality_from_effort(
                get_f(p, "catchability")?,
                get_f(p, "effort")?,
            ),
        )),
        "yield_per_recruit" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::yield_per_recruit(
                get_f(p, "f")?,
                get_f(p, "m")?,
                get_f(p, "w_inf")?,
                get_f(p, "k")?,
                get_f(p, "t_c")?,
                get_f(p, "t_r")?,
                get_f(p, "t0")?,
            ),
        )),
        "spawning_stock_biomass" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::spawning_stock_biomass(
                get_v(p, "numbers")?,
                get_v(p, "weights")?,
                get_v(p, "maturity")?,
            ),
        )),
        "exploitation_rate" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::exploitation_rate(get_f(p, "f")?, get_f(p, "z")?),
        )),
        "virtual_population_analysis" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::virtual_population_analysis(
                get_f(p, "catch")?,
                get_f(p, "m")?,
                get_f(p, "f_rate")?,
            ),
        )),
        "length_weight_relation" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::length_weight_relation(
                get_f(p, "length")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "condition_factor_fulton" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::condition_factor_fulton(
                get_f(p, "weight")?,
                get_f(p, "length")?,
            ),
        )),
        "surplus_production" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::surplus_production(
                get_f(p, "biomass_t")?,
                get_f(p, "biomass_t1")?,
                get_f(p, "catch")?,
            ),
        )),
        "fox_model_equilibrium_yield" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::fox_model_equilibrium_yield(
                get_f(p, "f")?,
                get_f(p, "msy")?,
                get_f(p, "f_msy")?,
            ),
        )),
        "selectivity_logistic" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::selectivity_logistic(
                get_f(p, "length")?,
                get_f(p, "l50")?,
                get_f(p, "slope")?,
            ),
        )),
        "discard_mortality" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::discard_mortality(
                get_f(p, "catch")?,
                get_f(p, "discard_fraction")?,
                get_f(p, "discard_survival")?,
            ),
        )),
        "osmoregulation_flux" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::osmoregulation_flux(
                get_f(p, "internal_osmolarity")?,
                get_f(p, "external_osmolarity")?,
                get_f(p, "permeability")?,
                get_f(p, "surface_area")?,
            ),
        )),
        "coral_bleaching_threshold" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::coral_bleaching_threshold(
                get_f(p, "sst")?,
                get_f(p, "mmm")?,
            ),
        )),
        "degree_heating_weeks" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::degree_heating_weeks(get_v(p, "weekly_anomalies")?),
        )),
        "ocean_acidification_ph" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::ocean_acidification_ph(
                get_f(p, "pco2")?,
                get_f(p, "alkalinity")?,
                get_f(p, "temperature")?,
            ),
        )),
        "carbonate_saturation_state" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::carbonate_saturation_state(
                get_f(p, "ca_concentration")?,
                get_f(p, "co3_concentration")?,
                get_f(p, "ksp")?,
            ),
        )),
        "bioluminescence_intensity" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::bioluminescence_intensity(
                get_f(p, "luciferin")?,
                get_f(p, "luciferase")?,
                get_f(p, "oxygen")?,
                get_f(p, "km")?,
            ),
        )),
        "depth_light_attenuation" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::depth_light_attenuation(
                get_f(p, "surface_irradiance")?,
                get_f(p, "attenuation_coeff")?,
                get_f(p, "depth")?,
            ),
        )),
        "thermohaline_density" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::thermohaline_density(
                get_f(p, "temperature")?,
                get_f(p, "salinity")?,
            ),
        )),
        "mixed_layer_depth_temperature" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::mixed_layer_depth_temperature(
                get_v(p, "profile_temps")?,
                get_v(p, "profile_depths")?,
                get_f(p, "threshold")?,
            ),
        )),
        "ekman_depth" => Ok(RunOutput::Scalar(bio::marine_biology::ocean::ekman_depth(
            get_f(p, "coriolis")?,
            get_f(p, "eddy_viscosity")?,
        ))),
        "ekman_transport" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::ekman_transport(
                get_f(p, "wind_stress")?,
                get_f(p, "coriolis")?,
                get_f(p, "density")?,
            ),
        )),
        "sverdrup_transport" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::sverdrup_transport(
                get_f(p, "wind_stress_curl")?,
                get_f(p, "beta")?,
            ),
        )),
        "primary_production_eppley" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::primary_production_eppley(
                get_f(p, "chlorophyll")?,
                get_f(p, "par")?,
                get_f(p, "temperature")?,
            ),
        )),
        "new_production_f_ratio" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::new_production_f_ratio(
                get_f(p, "nitrate_uptake")?,
                get_f(p, "total_uptake")?,
            ),
        )),
        "nitrogen_fixation_rate" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::nitrogen_fixation_rate(
                get_f(p, "temperature")?,
                get_f(p, "iron")?,
                get_f(p, "light")?,
            ),
        )),
        "oxygen_minimum_zone_depth" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::oxygen_minimum_zone_depth(
                get_f(p, "surface_o2")?,
                get_f(p, "respiration_rate")?,
                get_f(p, "ventilation_rate")?,
            ),
        )),
        "seawater_sound_speed" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::seawater_sound_speed(
                get_f(p, "temperature")?,
                get_f(p, "salinity")?,
                get_f(p, "depth")?,
            ),
        )),
        "coral_calcification_rate" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::coral_calcification_rate(
                get_f(p, "aragonite_saturation")?,
                get_f(p, "temperature")?,
                get_f(p, "light")?,
            ),
        )),
        "tidal_range" => Ok(RunOutput::Scalar(bio::marine_biology::ocean::tidal_range(
            get_f(p, "lunar_amplitude")?,
            get_f(p, "solar_amplitude")?,
            get_f(p, "phase_angle")?,
        ))),
        "wave_energy_density" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::wave_energy_density(
                get_f(p, "density")?,
                get_f(p, "gravity")?,
                get_f(p, "wave_height")?,
            ),
        )),
        "deep_water_wave_speed" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::deep_water_wave_speed(
                get_f(p, "gravity")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "upwelling_velocity" => Ok(RunOutput::Scalar(
            bio::marine_biology::ocean::upwelling_velocity(
                get_f(p, "wind_stress")?,
                get_f(p, "coriolis")?,
                get_f(p, "density")?,
                get_f(p, "coast_distance")?,
            ),
        )),
        "phytoplankton_growth" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::phytoplankton_growth(
                get_f(p, "mu_max")?,
                get_f(p, "nutrient")?,
                get_f(p, "ks")?,
                get_f(p, "light")?,
                get_f(p, "ki")?,
            ),
        )),
        "bloom_critical_depth" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::bloom_critical_depth(
                get_f(p, "surface_irradiance")?,
                get_f(p, "compensation_irradiance")?,
                get_f(p, "attenuation")?,
            ),
        )),
        "sverdrup_critical_depth" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::sverdrup_critical_depth(
                get_f(p, "avg_irradiance")?,
                get_f(p, "compensation_irradiance")?,
                get_f(p, "mixed_layer_depth")?,
                get_f(p, "attenuation")?,
            ),
        )),
        "nutrient_phytoplankton_zooplankton_step" => {
            let (a, b, c) = bio::marine_biology::plankton::nutrient_phytoplankton_zooplankton_step(
                get_f(p, "n")?,
                get_f(p, "p")?,
                get_f(p, "z")?,
                get_f(p, "dt")?,
                get_f(p, "mu")?,
                get_f(p, "ks")?,
                get_f(p, "grazing")?,
                get_f(p, "kp")?,
                get_f(p, "mortality_z")?,
                get_f(p, "recycling")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "chlorophyll_a_from_biomass" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::chlorophyll_a_from_biomass(
                get_f(p, "biomass")?,
                get_f(p, "carbon_to_chl_ratio")?,
            ),
        )),
        "zooplankton_diel_migration_depth" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::zooplankton_diel_migration_depth(
                get_f(p, "daytime_depth")?,
                get_f(p, "nighttime_depth")?,
                get_f(p, "time_hours")?,
            ),
        )),
        "harmful_algal_bloom_toxin" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::harmful_algal_bloom_toxin(
                get_f(p, "cell_density")?,
                get_f(p, "toxin_per_cell")?,
                get_f(p, "decay_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "redfield_ratio_deviation" => {
            let (a, b) = bio::marine_biology::plankton::redfield_ratio_deviation(
                get_f(p, "c")?,
                get_f(p, "n")?,
                get_f(p, "p")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "spring_bloom_timing" => Ok(RunOutput::Boolean(
            bio::marine_biology::plankton::spring_bloom_timing(
                get_f(p, "mixed_layer_depth")?,
                get_f(p, "critical_depth")?,
            ),
        )),
        "export_production" => Ok(RunOutput::Scalar(
            bio::marine_biology::plankton::export_production(
                get_f(p, "primary_production")?,
                get_f(p, "f_ratio")?,
            ),
        )),
        "trophic_transfer_efficiency" => Ok(RunOutput::Scalar(
            bio::marine_biology::fisheries::trophic_transfer_efficiency(
                get_f(p, "production_upper")?,
                get_f(p, "production_lower")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
