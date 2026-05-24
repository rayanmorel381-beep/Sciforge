//! Dispatch handler for nutrition functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "nutrient_absorption_first_order" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::nutrient_absorption_first_order(
                get_f(p, "dose")?,
                get_f(p, "ka")?,
                get_f(p, "t")?,
            ),
        )),
        "gastric_emptying" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::gastric_emptying(
                get_f(p, "volume")?,
                get_f(p, "half_life")?,
                get_f(p, "t")?,
            ),
        )),
        "glycemic_index_incremental_auc" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::glycemic_index_incremental_auc(
                get_v(p, "glucose_values")?,
                get_f(p, "baseline")?,
                get_f(p, "dt")?,
            ),
        )),
        "protein_digestibility_corrected_amino_acid_score" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::protein_digestibility_corrected_amino_acid_score(
                get_f(p, "limiting_aa_mg_g")?,
                get_f(p, "reference_mg_g")?,
                get_f(p, "digestibility")?,
            ),
        )),
        "nitrogen_balance" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::nitrogen_balance(
                get_f(p, "protein_intake_g")?,
                get_f(p, "urinary_n")?,
                get_f(p, "fecal_n")?,
                get_f(p, "sweat_n")?,
            ),
        )),
        "water_requirement_holliday_segar" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::water_requirement_holliday_segar(get_f(p, "weight_kg")?),
        )),
        "iron_absorption" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::iron_absorption(
                get_f(p, "non_heme_mg")?,
                get_f(p, "enhancers")?,
                get_f(p, "inhibitors")?,
                get_f(p, "heme_mg")?,
            ),
        )),
        "calcium_absorption_fraction" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::calcium_absorption_fraction(
                get_f(p, "intake_mg")?,
                get_f(p, "vitamin_d_nmol")?,
            ),
        )),
        "intestinal_transit_time" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::intestinal_transit_time(
                get_f(p, "fiber_g")?,
                get_f(p, "fluid_ml")?,
                get_f(p, "base_time_h")?,
            ),
        )),
        "nutrition_oral_bioavailability" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::oral_bioavailability(
                get_f(p, "fraction_absorbed")?,
                get_f(p, "gut_wall_extraction")?,
                get_f(p, "hepatic_extraction")?,
            ),
        )),
        "michaelis_menten_absorption" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::michaelis_menten_absorption(
                get_f(p, "vmax")?,
                get_f(p, "concentration")?,
                get_f(p, "km")?,
            ),
        )),
        "fat_soluble_vitamin_absorption" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::fat_soluble_vitamin_absorption(
                get_f(p, "dose")?,
                get_f(p, "fat_intake_g")?,
                get_f(p, "bile_salt_conc")?,
            ),
        )),
        "zinc_absorption_fraction" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::zinc_absorption_fraction(
                get_f(p, "intake_mg")?,
                get_f(p, "phytate_mg")?,
            ),
        )),
        "paracellular_absorption" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::paracellular_absorption(
                get_f(p, "permeability")?,
                get_f(p, "surface_area")?,
                get_f(p, "concentration")?,
            ),
        )),
        "glucose_transporter_kinetics" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::glucose_transporter_kinetics(
                get_f(p, "glucose")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "insulin_factor")?,
            ),
        )),
        "amino_acid_absorption_rate" => Ok(RunOutput::Scalar(
            bio::nutrition::absorption::amino_acid_absorption_rate(
                get_f(p, "concentration")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "competition_factor")?,
            ),
        )),
        "basal_metabolic_rate_mifflin" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::basal_metabolic_rate_mifflin(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
                get_f(p, "age")?,
                get_b(p, "is_male")?,
            ),
        )),
        "tdee" => Ok(RunOutput::Scalar(bio::nutrition::energy_balance::tdee(
            get_f(p, "bmr")?,
            get_f(p, "activity_factor")?,
            get_f(p, "thermic_effect")?,
        ))),
        "energy_balance" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::energy_balance(
                get_f(p, "intake_kcal")?,
                get_f(p, "expenditure_kcal")?,
            ),
        )),
        "weight_change_prediction" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::weight_change_prediction(
                get_f(p, "energy_balance_kcal_per_day")?,
                get_f(p, "days")?,
            ),
        )),
        "diet_induced_thermogenesis" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::diet_induced_thermogenesis(
                get_f(p, "protein_kcal")?,
                get_f(p, "carb_kcal")?,
                get_f(p, "fat_kcal")?,
            ),
        )),
        "respiratory_exchange_ratio" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::respiratory_exchange_ratio(
                get_f(p, "co2_produced")?,
                get_f(p, "o2_consumed")?,
            ),
        )),
        "substrate_oxidation_from_rer" => {
            let (a, b) =
                bio::nutrition::energy_balance::substrate_oxidation_from_rer(get_f(p, "rer")?);
            Ok(RunOutput::Pair(a, b))
        }
        "glycemic_index_load" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::glycemic_index_load(
                get_f(p, "gi")?,
                get_f(p, "carb_grams")?,
            ),
        )),
        "insulin_index_response" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::insulin_index_response(
                get_f(p, "glycemic_load")?,
                get_f(p, "protein_factor")?,
                get_f(p, "protein_grams")?,
            ),
        )),
        "body_composition_bmi" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::body_composition_bmi(
                get_f(p, "weight_kg")?,
                get_f(p, "height_m")?,
            ),
        )),
        "body_fat_percentage_navy" => Ok(RunOutput::Scalar(
            bio::nutrition::energy_balance::body_fat_percentage_navy(
                get_f(p, "waist_cm")?,
                get_f(p, "neck_cm")?,
                get_f(p, "height_cm")?,
                get_b(p, "is_male")?,
            ),
        )),
        "harris_benedict_male" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::harris_benedict_male(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
                get_f(p, "age")?,
            ),
        )),
        "harris_benedict_female" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::harris_benedict_female(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
                get_f(p, "age")?,
            ),
        )),
        "mifflin_st_jeor_male" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::mifflin_st_jeor_male(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
                get_f(p, "age")?,
            ),
        )),
        "mifflin_st_jeor_female" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::mifflin_st_jeor_female(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
                get_f(p, "age")?,
            ),
        )),
        "total_daily_energy_expenditure" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::total_daily_energy_expenditure(
                get_f(p, "bmr")?,
                get_f(p, "activity_factor")?,
            ),
        )),
        "thermic_effect_of_food" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::thermic_effect_of_food(
                get_f(p, "caloric_intake")?,
                get_f(p, "tef_fraction")?,
            ),
        )),
        "body_mass_index" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::body_mass_index(
                get_f(p, "weight_kg")?,
                get_f(p, "height_m")?,
            ),
        )),
        "lean_body_mass_boer_male" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::lean_body_mass_boer_male(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
            ),
        )),
        "lean_body_mass_boer_female" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::lean_body_mass_boer_female(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
            ),
        )),
        "body_fat_percentage" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::body_fat_percentage(
                get_f(p, "total_mass")?,
                get_f(p, "lean_mass")?,
            ),
        )),
        "katch_mcardle_bmr" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::katch_mcardle_bmr(get_f(p, "lean_body_mass_kg")?),
        )),
        "cunningham_bmr" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::cunningham_bmr(get_f(p, "lean_body_mass_kg")?),
        )),
        "nutrition_respiratory_quotient" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::respiratory_quotient(
                get_f(p, "co2_produced")?,
                get_f(p, "o2_consumed")?,
            ),
        )),
        "energy_from_macros" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::energy_from_macros(
                get_f(p, "carb_g")?,
                get_f(p, "protein_g")?,
                get_f(p, "fat_g")?,
                get_f(p, "alcohol_g")?,
            ),
        )),
        "waist_to_hip_ratio" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::waist_to_hip_ratio(
                get_f(p, "waist_cm")?,
                get_f(p, "hip_cm")?,
            ),
        )),
        "nutrition_body_surface_area_dubois" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::body_surface_area_dubois(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
            ),
        )),
        "ideal_body_weight_devine_male" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::ideal_body_weight_devine_male(get_f(p, "height_cm")?),
        )),
        "ideal_body_weight_devine_female" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::ideal_body_weight_devine_female(get_f(p, "height_cm")?),
        )),
        "adjusted_body_weight" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::adjusted_body_weight(
                get_f(p, "actual_kg")?,
                get_f(p, "ideal_kg")?,
            ),
        )),
        "resting_metabolic_rate_owen_male" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::resting_metabolic_rate_owen_male(get_f(p, "weight_kg")?),
        )),
        "resting_metabolic_rate_owen_female" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::resting_metabolic_rate_owen_female(get_f(p, "weight_kg")?),
        )),
        "glycemic_load" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::glycemic_load(
                get_f(p, "glycemic_index")?,
                get_f(p, "available_carbs_g")?,
            ),
        )),
        "fat_oxidation_rate" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::fat_oxidation_rate(get_f(p, "vo2")?, get_f(p, "vco2")?),
        )),
        "carb_oxidation_rate" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::carb_oxidation_rate(get_f(p, "vco2")?, get_f(p, "vo2")?),
        )),
        "protein_requirement_rda" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::protein_requirement_rda(get_f(p, "weight_kg")?),
        )),
        "estimated_average_requirement_calcium" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::estimated_average_requirement_calcium(get_f(p, "age")?),
        )),
        "recommended_daily_intake_scaling" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::recommended_daily_intake_scaling(
                get_f(p, "body_weight_kg")?,
                get_f(p, "rdi_per_kg")?,
            ),
        )),
        "vitamin_d_synthesis" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::vitamin_d_synthesis(
                get_f(p, "uvb_intensity")?,
                get_f(p, "skin_area")?,
                get_f(p, "melanin_factor")?,
                get_f(p, "exposure_minutes")?,
            ),
        )),
        "iron_absorption_enhancers" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::iron_absorption_enhancers(
                get_f(p, "non_heme_iron")?,
                get_f(p, "vitamin_c_mg")?,
                get_f(p, "meat_factor")?,
            ),
        )),
        "calcium_absorption" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::calcium_absorption(
                get_f(p, "intake")?,
                get_f(p, "vitamin_d")?,
                get_f(p, "age_factor")?,
            ),
        )),
        "zinc_copper_ratio" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::zinc_copper_ratio(
                get_f(p, "zinc_intake")?,
                get_f(p, "copper_intake")?,
            ),
        )),
        "bioavailability_factor" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::bioavailability_factor(
                get_f(p, "intake")?,
                get_f(p, "absorption_fraction")?,
                get_f(p, "first_pass_extraction")?,
            ),
        )),
        "folate_neural_tube_risk" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::folate_neural_tube_risk(
                get_f(p, "folate_ug")?,
                get_f(p, "risk_base")?,
                get_f(p, "protective_threshold")?,
            ),
        )),
        "omega3_omega6_ratio" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::omega3_omega6_ratio(
                get_f(p, "omega3")?,
                get_f(p, "omega6")?,
            ),
        )),
        "antioxidant_capacity_orac" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::antioxidant_capacity_orac(
                get_f(p, "concentration")?,
                get_f(p, "orac_per_unit")?,
            ),
        )),
        "iodine_thyroid_requirement" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::iodine_thyroid_requirement(
                get_f(p, "body_weight_kg")?,
                get_f(p, "base_requirement_ug_per_kg")?,
                get_f(p, "pregnancy_factor")?,
            ),
        )),
        "nutrient_density_score" => Ok(RunOutput::Scalar(
            bio::nutrition::micronutrients::nutrient_density_score(
                get_v(p, "nutrients")?,
                get_v(p, "daily_values")?,
                get_f(p, "calories")?,
            ),
        )),
        "body_surface_area_dubois" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::body_surface_area_dubois(
                get_f(p, "weight_kg")?,
                get_f(p, "height_cm")?,
            ),
        )),
        "respiratory_quotient" => Ok(RunOutput::Scalar(
            bio::nutrition::metabolism::respiratory_quotient(
                get_f(p, "co2_produced")?,
                get_f(p, "o2_consumed")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
