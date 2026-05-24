//! Dispatch handler for plant biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "competitive_exclusion_tilman" => Ok(RunOutput::Text(
            bio::plant_biology::ecology::competitive_exclusion_tilman(
                get_f(p, "r_star_a")?,
                get_f(p, "r_star_b")?,
            )
            .to_string(),
        )),
        "allelopathy_effect" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::allelopathy_effect(
                get_f(p, "allelochemical_conc")?,
                get_f(p, "ic50")?,
                get_f(p, "max_inhibition")?,
            ),
        )),
        "light_competition_beer_lambert" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::light_competition_beer_lambert(
                get_f(p, "light_above")?,
                get_f(p, "lai")?,
                get_f(p, "extinction_coeff")?,
            ),
        )),
        "canopy_lai" => Ok(RunOutput::Scalar(bio::plant_biology::ecology::canopy_lai(
            get_f(p, "leaf_area")?,
            get_f(p, "ground_area")?,
        ))),
        "sla" => Ok(RunOutput::Scalar(bio::plant_biology::ecology::sla(
            get_f(p, "leaf_area")?,
            get_f(p, "leaf_dry_mass")?,
        ))),
        "plant_defense_investment" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::plant_defense_investment(
                get_f(p, "growth_rate")?,
                get_f(p, "defense_allocation")?,
            ),
        )),
        "herbivory_damage" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::herbivory_damage(
                get_f(p, "herbivore_density")?,
                get_f(p, "feeding_rate")?,
                get_f(p, "plant_biomass")?,
                get_f(p, "defense_level")?,
            ),
        )),
        "seed_dispersal_kernel" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::seed_dispersal_kernel(
                get_f(p, "distance")?,
                get_f(p, "mean_dispersal")?,
            ),
        )),
        "pollination_success" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::pollination_success(
                get_f(p, "pollinator_visits")?,
                get_f(p, "pollen_per_visit")?,
                get_f(p, "ovule_count")?,
            ),
        )),
        "nitrogen_fixation_symbiotic" => Ok(RunOutput::Scalar(
            bio::plant_biology::ecology::nitrogen_fixation_symbiotic(
                get_f(p, "nodule_mass")?,
                get_f(p, "nitrogenase_activity")?,
                get_f(p, "oxygen_limitation")?,
            ),
        )),
        "root_growth_logistic" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::root_growth_logistic(
                get_f(p, "length")?,
                get_f(p, "max_length")?,
                get_f(p, "rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "auxin_gradient" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::auxin_gradient(
                get_f(p, "source_concentration")?,
                get_f(p, "distance")?,
                get_f(p, "diffusion")?,
                get_f(p, "decay")?,
            ),
        )),
        "phototropism_bending_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::phototropism_bending_rate(
                get_f(p, "light_differential")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "gravitropism_response" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::gravitropism_response(
                get_f(p, "angle")?,
                get_f(p, "sensitivity")?,
                get_f(p, "dt")?,
            ),
        )),
        "leaf_area_index" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::leaf_area_index(
                get_f(p, "total_leaf_area")?,
                get_f(p, "ground_area")?,
            ),
        )),
        "beer_lambert_canopy" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::beer_lambert_canopy(
                get_f(p, "light_above")?,
                get_f(p, "k")?,
                get_f(p, "lai")?,
            ),
        )),
        "thermal_time" => Ok(RunOutput::Scalar(bio::plant_biology::growth::thermal_time(
            get_f(p, "daily_mean_temp")?,
            get_f(p, "base_temp")?,
        ))),
        "water_potential" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::water_potential(
                get_f(p, "osmotic")?,
                get_f(p, "pressure")?,
                get_f(p, "gravitational")?,
            ),
        )),
        "xylem_flow_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::xylem_flow_rate(
                get_f(p, "pressure_gradient")?,
                get_f(p, "conductivity")?,
                get_f(p, "cross_section")?,
            ),
        )),
        "phloem_transport_munch" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::phloem_transport_munch(
                get_f(p, "source_pressure")?,
                get_f(p, "sink_pressure")?,
                get_f(p, "resistance")?,
            ),
        )),
        "allometric_biomass" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::allometric_biomass(
                get_f(p, "diameter")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "specific_leaf_area" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::specific_leaf_area(
                get_f(p, "leaf_area")?,
                get_f(p, "leaf_dry_mass")?,
            ),
        )),
        "relative_growth_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::relative_growth_rate(
                get_f(p, "biomass_initial")?,
                get_f(p, "biomass_final")?,
                get_f(p, "time")?,
            ),
        )),
        "net_assimilation_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::net_assimilation_rate(
                get_f(p, "biomass_change")?,
                get_f(p, "leaf_area_avg")?,
                get_f(p, "time")?,
            ),
        )),
        "phytochrome_response" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::phytochrome_response(
                get_f(p, "red")?,
                get_f(p, "far_red")?,
            ),
        )),
        "vernalization_progress" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::vernalization_progress(
                get_f(p, "temp")?,
                get_f(p, "optimal_temp")?,
                get_f(p, "range")?,
                get_f(p, "dt")?,
            ),
        )),
        "photoperiod_response" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::photoperiod_response(
                get_f(p, "day_length")?,
                get_f(p, "critical_length")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "root_shoot_ratio" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::root_shoot_ratio(
                get_f(p, "root_biomass")?,
                get_f(p, "shoot_biomass")?,
            ),
        )),
        "canopy_gap_fraction" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::canopy_gap_fraction(get_f(p, "lai")?, get_f(p, "k")?),
        )),
        "stem_taper" => Ok(RunOutput::Scalar(bio::plant_biology::growth::stem_taper(
            get_f(p, "diameter_base")?,
            get_f(p, "height_fraction")?,
            get_f(p, "taper_exponent")?,
        ))),
        "cavitation_vulnerability" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::cavitation_vulnerability(
                get_f(p, "pressure")?,
                get_f(p, "p50")?,
                get_f(p, "slope")?,
            ),
        )),
        "turgor_pressure" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::turgor_pressure(
                get_f(p, "osmotic_potential")?,
                get_f(p, "water_potential")?,
            ),
        )),
        "gibberellin_stem_elongation" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::gibberellin_stem_elongation(
                get_f(p, "ga_concentration")?,
                get_f(p, "max_rate")?,
                get_f(p, "km")?,
            ),
        )),
        "senescence_chlorophyll_loss" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::senescence_chlorophyll_loss(
                get_f(p, "chl0")?,
                get_f(p, "degradation_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "frost_hardiness" => Ok(RunOutput::Scalar(
            bio::plant_biology::growth::frost_hardiness(
                get_f(p, "temp")?,
                get_f(p, "lt50")?,
                get_f(p, "slope")?,
            ),
        )),
        "farquhar_c3" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::farquhar_c3(
                get_f(p, "vcmax")?,
                get_f(p, "ci")?,
                get_f(p, "gamma_star")?,
                get_f(p, "kc")?,
                get_f(p, "ko")?,
                get_f(p, "o")?,
                get_f(p, "j")?,
                get_f(p, "rd")?,
            ),
        )),
        "plant_light_response_curve" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::light_response_curve(
                get_f(p, "phi")?,
                get_f(p, "ppfd")?,
                get_f(p, "amax")?,
                get_f(p, "theta")?,
                get_f(p, "rd")?,
            ),
        )),
        "transpiration_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::transpiration_rate(
                get_f(p, "stomatal_conductance")?,
                get_f(p, "vpd")?,
            ),
        )),
        "ball_berry_conductance" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::ball_berry_conductance(
                get_f(p, "a_n")?,
                get_f(p, "cs")?,
                get_f(p, "rh")?,
                get_f(p, "g0")?,
                get_f(p, "g1")?,
            ),
        )),
        "plant_water_use_efficiency" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::water_use_efficiency(
                get_f(p, "a_n")?,
                get_f(p, "transpiration")?,
            ),
        )),
        "penman_monteith" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::penman_monteith(
                get_f(p, "net_radiation")?,
                get_f(p, "soil_heat")?,
                get_f(p, "vpd")?,
                get_f(p, "ga")?,
                get_f(p, "gs")?,
                get_f(p, "delta")?,
                get_f(p, "gamma")?,
                get_f(p, "rho")?,
                get_f(p, "cp")?,
            ),
        )),
        "plant_rubisco_specificity" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::rubisco_specificity(
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
                get_f(p, "vomax")?,
                get_f(p, "ko")?,
            ),
        )),
        "plant_photorespiration_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::photorespiration_rate(
                get_f(p, "vomax")?,
                get_f(p, "o")?,
                get_f(p, "ko")?,
                get_f(p, "ci")?,
                get_f(p, "kc")?,
            ),
        )),
        "plant_electron_transport_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::electron_transport_rate(
                get_f(p, "ppfd")?,
                get_f(p, "absorptance")?,
                get_f(p, "fraction_psii")?,
                get_f(p, "phi_psii")?,
            ),
        )),
        "stomatal_optimization" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::stomatal_optimization(
                get_f(p, "vpd")?,
                get_f(p, "ca")?,
                get_f(p, "lambda_wue")?,
                get_f(p, "g1")?,
            ),
        )),
        "c4_photosynthesis" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::c4_photosynthesis(
                get_f(p, "vpmax")?,
                get_f(p, "ci")?,
                get_f(p, "kp")?,
                get_f(p, "vcmax")?,
                get_f(p, "ko")?,
                get_f(p, "kc")?,
                get_f(p, "o")?,
                get_f(p, "rd")?,
            ),
        )),
        "cam_malic_acid_storage" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::cam_malic_acid_storage(
                get_f(p, "co2_fixed_night")?,
                get_f(p, "vacuole_capacity")?,
                get_f(p, "current_malate")?,
            ),
        )),
        "cam_daytime_decarboxylation" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::cam_daytime_decarboxylation(
                get_f(p, "malate")?,
                get_f(p, "decarboxylation_rate")?,
            ),
        )),
        "chlorophyll_fluorescence_fv_fm" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::chlorophyll_fluorescence_fv_fm(
                get_f(p, "f0")?,
                get_f(p, "fm")?,
            ),
        )),
        "non_photochemical_quenching" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::non_photochemical_quenching(
                get_f(p, "fm")?,
                get_f(p, "fm_prime")?,
            ),
        )),
        "photochemical_quenching" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::photochemical_quenching(
                get_f(p, "fs")?,
                get_f(p, "f0_prime")?,
                get_f(p, "fm_prime")?,
            ),
        )),
        "quantum_yield_psii" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::quantum_yield_psii(
                get_f(p, "phi_psii")?,
                get_f(p, "ppfd")?,
            ),
        )),
        "co2_compensation_point" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::co2_compensation_point(
                get_f(p, "gamma_star")?,
                get_f(p, "rd")?,
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
                get_f(p, "ko")?,
                get_f(p, "o")?,
            ),
        )),
        "mesophyll_conductance" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::mesophyll_conductance(
                get_f(p, "a_n")?,
                get_f(p, "ci")?,
                get_f(p, "cc")?,
            ),
        )),
        "light_use_efficiency" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::light_use_efficiency(
                get_f(p, "gpp")?,
                get_f(p, "apar")?,
            ),
        )),
        "vcmax_temperature_response" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::vcmax_temperature_response(
                get_f(p, "vcmax25")?,
                get_f(p, "ha")?,
                get_f(p, "temp_k")?,
            ),
        )),
        "jmax_temperature_peaked" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::jmax_temperature_peaked(
                get_f(p, "jmax25")?,
                get_f(p, "ha")?,
                get_f(p, "hd")?,
                get_f(p, "ds")?,
                get_f(p, "temp_k")?,
            ),
        )),
        "xylem_flow_hagen_poiseuille" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::xylem_flow_hagen_poiseuille(
                get_f(p, "radius")?,
                get_f(p, "pressure_gradient")?,
                get_f(p, "viscosity")?,
                get_f(p, "length")?,
            ),
        )),
        "leaf_transpiration_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::leaf_transpiration_rate(
                get_f(p, "stomatal_conductance")?,
                get_f(p, "vpd")?,
            ),
        )),
        "cohesion_tension_water_potential" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::cohesion_tension_water_potential(
                get_f(p, "osmotic")?,
                get_f(p, "pressure")?,
                get_f(p, "gravity")?,
                get_f(p, "matric")?,
            ),
        )),
        "phloem_munch_flow" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::phloem_munch_flow(
                get_f(p, "turgor_source")?,
                get_f(p, "turgor_sink")?,
                get_f(p, "resistance")?,
            ),
        )),
        "root_water_uptake" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::root_water_uptake(
                get_f(p, "soil_water_potential")?,
                get_f(p, "root_water_potential")?,
                get_f(p, "root_conductance")?,
            ),
        )),
        "xylem_cavitation_vulnerability" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::xylem_cavitation_vulnerability(
                get_f(p, "water_potential")?,
                get_f(p, "p50")?,
                get_f(p, "slope")?,
            ),
        )),
        "plant_stomatal_conductance_ball_berry" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::stomatal_conductance_ball_berry(
                get_f(p, "assimilation")?,
                get_f(p, "humidity")?,
                get_f(p, "co2_surface")?,
                get_f(p, "g0")?,
                get_f(p, "g1")?,
            ),
        )),
        "sugar_loading_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::sugar_loading_rate(
                get_f(p, "sucrose_conc")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
            ),
        )),
        "root_hydraulic_conductivity" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::root_hydraulic_conductivity(
                get_f(p, "flow_rate")?,
                get_f(p, "root_surface_area")?,
                get_f(p, "pressure_difference")?,
            ),
        )),
        "sap_flow_heat_pulse" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::sap_flow_heat_pulse(
                get_f(p, "thermal_diffusivity")?,
                get_f(p, "heat_pulse_distance")?,
                get_f(p, "time_to_max")?,
            ),
        )),
        "electron_transport_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::electron_transport_rate(
                get_f(p, "ppfd")?,
                get_f(p, "absorptance")?,
                get_f(p, "fraction_psii")?,
                get_f(p, "phi_psii")?,
            ),
        )),
        "light_response_curve" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::light_response_curve(
                get_f(p, "phi")?,
                get_f(p, "ppfd")?,
                get_f(p, "amax")?,
                get_f(p, "theta")?,
                get_f(p, "rd")?,
            ),
        )),
        "photorespiration_rate" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::photorespiration_rate(
                get_f(p, "vomax")?,
                get_f(p, "o")?,
                get_f(p, "ko")?,
                get_f(p, "ci")?,
                get_f(p, "kc")?,
            ),
        )),
        "rubisco_specificity" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::rubisco_specificity(
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
                get_f(p, "vomax")?,
                get_f(p, "ko")?,
            ),
        )),
        "stomatal_conductance_ball_berry" => Ok(RunOutput::Scalar(
            bio::plant_biology::transport::stomatal_conductance_ball_berry(
                get_f(p, "assimilation")?,
                get_f(p, "humidity")?,
                get_f(p, "co2_surface")?,
                get_f(p, "g0")?,
                get_f(p, "g1")?,
            ),
        )),
        "water_use_efficiency" => Ok(RunOutput::Scalar(
            bio::plant_biology::photosynthesis::water_use_efficiency(
                get_f(p, "a_n")?,
                get_f(p, "transpiration")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
