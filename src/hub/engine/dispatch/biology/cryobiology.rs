//! Dispatch handler for cryobiology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "mazur_two_factor_model" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::mazur_two_factor_model(
                get_f(p, "cooling_rate")?,
                get_f(p, "optimal_rate")?,
                get_f(p, "width")?,
            ),
        )),
        "ice_nucleation_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::ice_nucleation_rate(
                get_f(p, "temperature")?,
                get_f(p, "volume")?,
                get_f(p, "j0")?,
                get_f(p, "activation_energy")?,
            ),
        )),
        "critical_cooling_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::critical_cooling_rate(
                get_f(p, "cpa_concentration")?,
                get_f(p, "base_rate")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "vitrification_probability" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::vitrification_probability(
                get_f(p, "cooling_rate")?,
                get_f(p, "critical_rate")?,
            ),
        )),
        "cpa_toxicity" => Ok(RunOutput::Scalar(bio::cryobiology::freezing::cpa_toxicity(
            get_f(p, "concentration")?,
            get_f(p, "exposure_time")?,
            get_f(p, "k_tox")?,
        ))),
        "cell_volume_response" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::cell_volume_response(
                get_f(p, "v0")?,
                get_f(p, "osmolarity_ratio")?,
                get_f(p, "vb")?,
            ),
        )),
        "freeze_thaw_survival" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::freeze_thaw_survival(
                get_f(p, "initial_viability")?,
                get_f(p, "ice_damage")?,
                get_f(p, "osmotic_damage")?,
                get_f(p, "cpa_damage")?,
            ),
        )),
        "intracellular_ice_formation_probability" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::intracellular_ice_formation_probability(
                get_f(p, "cooling_rate")?,
                get_f(p, "critical_rate")?,
                get_f(p, "n")?,
            ),
        )),
        "osmotic_tolerance_limit" => {
            let (a, b) = bio::cryobiology::freezing::osmotic_tolerance_limit(
                get_f(p, "v_min")?,
                get_f(p, "v_max")?,
                get_f(p, "initial_volume")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "kedem_katchalsky_water_flux" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::kedem_katchalsky_water_flux(
                get_f(p, "lp")?,
                get_f(p, "area")?,
                get_f(p, "delta_pi")?,
                get_f(p, "sigma")?,
                get_f(p, "delta_p")?,
            ),
        )),
        "kedem_katchalsky_solute_flux" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::kedem_katchalsky_solute_flux(
                get_f(p, "ps")?,
                get_f(p, "area")?,
                get_f(p, "delta_c")?,
                get_f(p, "sigma")?,
                get_f(p, "jv")?,
                get_f(p, "c_mean")?,
            ),
        )),
        "freezing_point_depression" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::freezing_point_depression(
                get_f(p, "concentration")?,
                get_f(p, "kf")?,
                get_f(p, "dissociation_factor")?,
            ),
        )),
        "hemolysis_fraction" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::hemolysis_fraction(
                get_f(p, "osmolality")?,
                get_f(p, "half_lysis_osmolality")?,
                get_f(p, "steepness")?,
            ),
        )),
        "stefan_freezing_front" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::stefan_freezing_front(
                get_f(p, "thermal_diffusivity")?,
                get_f(p, "t")?,
                get_f(p, "stefan_number")?,
            ),
        )),
        "supercooling_degree" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::supercooling_degree(
                get_f(p, "freezing_point")?,
                get_f(p, "nucleation_temp")?,
            ),
        )),
        "ice_crystal_growth_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::ice_crystal_growth_rate(
                get_f(p, "supercooling")?,
                get_f(p, "diffusivity")?,
                get_f(p, "activation_energy")?,
                get_f(p, "temperature")?,
            ),
        )),
        "cpa_loading_protocol_step" => {
            let (a, b) = bio::cryobiology::freezing::cpa_loading_protocol_step(
                get_f(p, "v")?,
                get_f(p, "lp")?,
                get_f(p, "area")?,
                get_f(p, "osm_in")?,
                get_f(p, "osm_out")?,
                get_f(p, "ps")?,
                get_f(p, "c_in")?,
                get_f(p, "c_out")?,
                get_f(p, "vb")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "rewarming_crystallization_risk" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::rewarming_crystallization_risk(
                get_f(p, "warming_rate")?,
                get_f(p, "critical_warming")?,
            ),
        )),
        "glass_transition_temperature" => Ok(RunOutput::Scalar(
            bio::cryobiology::freezing::glass_transition_temperature(
                get_f(p, "cpa_fraction")?,
                get_f(p, "tg_cpa")?,
                get_f(p, "tg_water")?,
            ),
        )),
        "nucleation_temperature" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::nucleation_temperature(
                get_f(p, "solution_concentration")?,
                get_f(p, "cooling_rate")?,
            ),
        )),
        "ice_growth_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::ice_growth_rate(
                get_f(p, "supercooling")?,
                get_f(p, "diffusion_coeff")?,
                get_f(p, "latent_heat")?,
            ),
        )),
        "intracellular_ice_probability" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::intracellular_ice_probability(
                get_f(p, "cooling_rate")?,
                get_f(p, "critical_rate")?,
            ),
        )),
        "ostwald_recrystallization_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::ostwald_recrystallization_rate(
                get_f(p, "temperature")?,
                get_f(p, "activation_energy")?,
            ),
        )),
        "anti_freeze_protein_thermal_hysteresis" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::anti_freeze_protein_thermal_hysteresis(
                get_f(p, "concentration")?,
                get_f(p, "k_th")?,
                get_f(p, "n")?,
            ),
        )),
        "cryoprotectant_toxicity" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::cryoprotectant_toxicity(
                get_f(p, "concentration")?,
                get_f(p, "temperature")?,
                get_f(p, "exposure_time")?,
                get_f(p, "k_tox")?,
            ),
        )),
        "dehydration_during_freezing" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::dehydration_during_freezing(
                get_f(p, "initial_water")?,
                get_f(p, "osmotic_coefficient")?,
                get_f(p, "ice_fraction")?,
            ),
        )),
        "vitrification_temperature" => Ok(RunOutput::Scalar(
            bio::cryobiology::ice_formation::vitrification_temperature(
                get_f(p, "water_fraction")?,
                get_f(p, "tg_pure_solute")?,
                get_f(p, "tg_water")?,
                get_f(p, "k_gt")?,
            ),
        )),
        "storage_decay_arrhenius" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::storage_decay_arrhenius(
                get_f(p, "a")?,
                get_f(p, "ea")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "shelf_life" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::shelf_life(
                get_f(p, "initial_viability")?,
                get_f(p, "threshold")?,
                get_f(p, "decay_rate")?,
            ),
        )),
        "recrystallization_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::recrystallization_rate(
                get_f(p, "temperature")?,
                get_f(p, "activation_energy")?,
                get_f(p, "pre_factor")?,
            ),
        )),
        "warming_rate_survival" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::warming_rate_survival(
                get_f(p, "warming_rate")?,
                get_f(p, "optimal_warming")?,
                get_f(p, "sigma")?,
            ),
        )),
        "devitrification_probability" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::devitrification_probability(
                get_f(p, "warming_rate")?,
                get_f(p, "critical_warming_rate")?,
            ),
        )),
        "cpa_permeation" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::cpa_permeation(
                get_f(p, "permeability")?,
                get_f(p, "surface_area")?,
                get_f(p, "concentration_out")?,
                get_f(p, "concentration_in")?,
            ),
        )),
        "two_parameter_model_volume" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::two_parameter_model_volume(
                get_f(p, "volume0")?,
                get_f(p, "lp")?,
                get_f(p, "surface_area")?,
                get_f(p, "osm_out")?,
                get_f(p, "osm_in")?,
                get_f(p, "dt")?,
            ),
        )),
        "cooling_rate_survival" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::cooling_rate_survival(
                get_f(p, "cooling_rate")?,
                get_f(p, "optimal")?,
                get_f(p, "sigma")?,
            ),
        )),
        "ice_nucleation_probability" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::ice_nucleation_probability(
                get_f(p, "temperature")?,
                get_f(p, "volume")?,
                get_f(p, "j0")?,
                get_f(p, "ea")?,
            ),
        )),
        "lyophilization_primary_drying_rate" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::lyophilization_primary_drying_rate(
                get_f(p, "heat_input")?,
                get_f(p, "sublimation_enthalpy")?,
            ),
        )),
        "lyophilization_collapse_temperature" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::lyophilization_collapse_temperature(
                get_f(p, "tg_prime")?,
                get_f(p, "offset")?,
            ),
        )),
        "trehalose_protection" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::trehalose_protection(
                get_f(p, "trehalose_conc")?,
                get_f(p, "k_protect")?,
                get_f(p, "max_protection")?,
            ),
        )),
        "thawing_temperature_profile" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::thawing_temperature_profile(
                get_f(p, "t_initial")?,
                get_f(p, "t_bath")?,
                get_f(p, "k")?,
                get_f(p, "time")?,
            ),
        )),
        "post_thaw_recovery_kinetics" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::post_thaw_recovery_kinetics(
                get_f(p, "plateau")?,
                get_f(p, "recovery_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "controlled_rate_freezer_program" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::controlled_rate_freezer_program(
                get_f(p, "target_rate")?,
                get_f(p, "current_temp")?,
                get_f(p, "dt")?,
            ),
        )),
        "thermal_seed_temperature" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::thermal_seed_temperature(
                get_f(p, "sample_temp")?,
                get_f(p, "seed_offset")?,
            ),
        )),
        "isochoric_preservation_pressure" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::isochoric_preservation_pressure(
                get_f(p, "temperature")?,
                get_f(p, "reference_temp")?,
                get_f(p, "bulk_modulus")?,
                get_f(p, "expansion_coeff")?,
            ),
        )),
        "q10_temperature_coefficient" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::q10_temperature_coefficient(
                get_f(p, "rate_t2")?,
                get_f(p, "rate_t1")?,
                get_f(p, "t2")?,
                get_f(p, "t1")?,
            ),
        )),
        "wlf_viscosity_shift" => Ok(RunOutput::Scalar(
            bio::cryobiology::preservation::wlf_viscosity_shift(
                get_f(p, "c1")?,
                get_f(p, "c2")?,
                get_f(p, "temperature")?,
                get_f(p, "tg")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
