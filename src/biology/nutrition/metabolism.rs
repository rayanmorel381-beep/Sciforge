pub fn harris_benedict_male(weight_kg: f64, height_cm: f64, age: f64) -> f64 {
    88.362 + 13.397 * weight_kg + 4.799 * height_cm - 5.677 * age
}

pub fn harris_benedict_female(weight_kg: f64, height_cm: f64, age: f64) -> f64 {
    447.593 + 9.247 * weight_kg + 3.098 * height_cm - 4.330 * age
}

pub fn mifflin_st_jeor_male(weight_kg: f64, height_cm: f64, age: f64) -> f64 {
    10.0 * weight_kg + 6.25 * height_cm - 5.0 * age + 5.0
}

pub fn mifflin_st_jeor_female(weight_kg: f64, height_cm: f64, age: f64) -> f64 {
    10.0 * weight_kg + 6.25 * height_cm - 5.0 * age - 161.0
}

pub fn total_daily_energy_expenditure(bmr: f64, activity_factor: f64) -> f64 {
    bmr * activity_factor
}

pub fn thermic_effect_of_food(caloric_intake: f64, tef_fraction: f64) -> f64 {
    caloric_intake * tef_fraction
}

pub fn body_mass_index(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

pub fn lean_body_mass_boer_male(weight_kg: f64, height_cm: f64) -> f64 {
    0.407 * weight_kg + 0.267 * height_cm - 19.2
}

pub fn lean_body_mass_boer_female(weight_kg: f64, height_cm: f64) -> f64 {
    0.252 * weight_kg + 0.473 * height_cm - 48.3
}

pub fn body_fat_percentage(total_mass: f64, lean_mass: f64) -> f64 {
    (total_mass - lean_mass) / total_mass * 100.0
}

pub fn katch_mcardle_bmr(lean_body_mass_kg: f64) -> f64 {
    370.0 + 21.6 * lean_body_mass_kg
}

pub fn cunningham_bmr(lean_body_mass_kg: f64) -> f64 {
    500.0 + 22.0 * lean_body_mass_kg
}

pub fn respiratory_quotient(co2_produced: f64, o2_consumed: f64) -> f64 {
    co2_produced / o2_consumed
}

pub fn energy_from_macros(carb_g: f64, protein_g: f64, fat_g: f64, alcohol_g: f64) -> f64 {
    carb_g * 4.0 + protein_g * 4.0 + fat_g * 9.0 + alcohol_g * 7.0
}

pub fn waist_to_hip_ratio(waist_cm: f64, hip_cm: f64) -> f64 {
    waist_cm / hip_cm
}

pub fn body_surface_area_dubois(weight_kg: f64, height_cm: f64) -> f64 {
    0.007184 * weight_kg.powf(0.425) * height_cm.powf(0.725)
}

pub fn ideal_body_weight_devine_male(height_cm: f64) -> f64 {
    50.0 + 2.3 * ((height_cm / 2.54) - 60.0)
}

pub fn ideal_body_weight_devine_female(height_cm: f64) -> f64 {
    45.5 + 2.3 * ((height_cm / 2.54) - 60.0)
}

pub fn adjusted_body_weight(actual_kg: f64, ideal_kg: f64) -> f64 {
    ideal_kg + 0.4 * (actual_kg - ideal_kg)
}

pub fn resting_metabolic_rate_owen_male(weight_kg: f64) -> f64 {
    879.0 + 10.2 * weight_kg
}

pub fn resting_metabolic_rate_owen_female(weight_kg: f64) -> f64 {
    795.0 + 7.18 * weight_kg
}

pub fn glycemic_load(glycemic_index: f64, available_carbs_g: f64) -> f64 {
    glycemic_index * available_carbs_g / 100.0
}

pub fn fat_oxidation_rate(vo2: f64, vco2: f64) -> f64 {
    1.67 * vo2 - 1.67 * vco2
}

pub fn carb_oxidation_rate(vco2: f64, vo2: f64) -> f64 {
    4.55 * vco2 - 3.21 * vo2
}

pub fn protein_requirement_rda(weight_kg: f64) -> f64 {
    0.8 * weight_kg
}

pub fn estimated_average_requirement_calcium(age: f64) -> f64 {
    if age < 1.0 {
        260.0
    } else if age < 4.0 {
        500.0
    } else if age < 9.0 {
        800.0
    } else if age < 19.0 {
        1100.0
    } else if age < 51.0 {
        800.0
    } else {
        1000.0
    }
}
