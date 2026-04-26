pub fn basal_metabolic_rate_mifflin(
    weight_kg: f64,
    height_cm: f64,
    age: f64,
    is_male: bool,
) -> f64 {
    if is_male {
        10.0 * weight_kg + 6.25 * height_cm - 5.0 * age + 5.0
    } else {
        10.0 * weight_kg + 6.25 * height_cm - 5.0 * age - 161.0
    }
}

pub fn tdee(bmr: f64, activity_factor: f64, thermic_effect: f64) -> f64 {
    bmr * activity_factor + thermic_effect
}

pub fn energy_balance(intake_kcal: f64, expenditure_kcal: f64) -> f64 {
    intake_kcal - expenditure_kcal
}

pub fn weight_change_prediction(energy_balance_kcal_per_day: f64, days: f64) -> f64 {
    energy_balance_kcal_per_day * days / 7700.0
}

pub fn diet_induced_thermogenesis(protein_kcal: f64, carb_kcal: f64, fat_kcal: f64) -> f64 {
    protein_kcal * 0.25 + carb_kcal * 0.08 + fat_kcal * 0.03
}

pub fn respiratory_exchange_ratio(co2_produced: f64, o2_consumed: f64) -> f64 {
    co2_produced / o2_consumed.max(1e-30)
}

pub fn substrate_oxidation_from_rer(rer: f64) -> (f64, f64) {
    let fat_fraction = (1.0 - rer) / 0.29;
    let carb_fraction = (rer - 0.71) / 0.29;
    (carb_fraction.clamp(0.0, 1.0), fat_fraction.clamp(0.0, 1.0))
}

pub fn glycemic_index_load(gi: f64, carb_grams: f64) -> f64 {
    gi * carb_grams / 100.0
}

pub fn insulin_index_response(glycemic_load: f64, protein_factor: f64, protein_grams: f64) -> f64 {
    glycemic_load + protein_factor * protein_grams
}

pub fn body_composition_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m).max(1e-30)
}

pub fn body_fat_percentage_navy(waist_cm: f64, neck_cm: f64, height_cm: f64, is_male: bool) -> f64 {
    if is_male {
        495.0
            / (1.0324 - 0.19077 * (waist_cm - neck_cm).max(1e-30).log10()
                + 0.15456 * height_cm.max(1e-30).log10())
            - 450.0
    } else {
        495.0
            / (1.29579 - 0.35004 * (waist_cm + 0.0 - neck_cm).max(1e-30).log10()
                + 0.22100 * height_cm.max(1e-30).log10())
            - 450.0
    }
}
