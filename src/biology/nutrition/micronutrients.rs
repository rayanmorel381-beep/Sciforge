pub fn recommended_daily_intake_scaling(body_weight_kg: f64, rdi_per_kg: f64) -> f64 {
    body_weight_kg * rdi_per_kg
}

pub fn vitamin_d_synthesis(
    uvb_intensity: f64,
    skin_area: f64,
    melanin_factor: f64,
    exposure_minutes: f64,
) -> f64 {
    uvb_intensity * skin_area * (1.0 - melanin_factor) * exposure_minutes / 60.0
}

pub fn iron_absorption_enhancers(non_heme_iron: f64, vitamin_c_mg: f64, meat_factor: f64) -> f64 {
    let vc_factor = 1.0 + 0.01 * vitamin_c_mg;
    non_heme_iron * 0.05 * vc_factor * (1.0 + meat_factor)
}

pub fn calcium_absorption(intake: f64, vitamin_d: f64, age_factor: f64) -> f64 {
    let base_absorption = 0.3;
    let vd_factor = vitamin_d / (vitamin_d + 20.0);
    intake * base_absorption * (1.0 + vd_factor) * age_factor
}

pub fn zinc_copper_ratio(zinc_intake: f64, copper_intake: f64) -> f64 {
    zinc_intake / copper_intake.max(1e-30)
}

pub fn bioavailability_factor(
    intake: f64,
    absorption_fraction: f64,
    first_pass_extraction: f64,
) -> f64 {
    intake * absorption_fraction * (1.0 - first_pass_extraction)
}

pub fn folate_neural_tube_risk(folate_ug: f64, risk_base: f64, protective_threshold: f64) -> f64 {
    if folate_ug >= protective_threshold {
        return risk_base * 0.3;
    }
    risk_base * (1.0 - 0.7 * folate_ug / protective_threshold)
}

pub fn omega3_omega6_ratio(omega3: f64, omega6: f64) -> f64 {
    omega3 / omega6.max(1e-30)
}

pub fn antioxidant_capacity_orac(concentration: f64, orac_per_unit: f64) -> f64 {
    concentration * orac_per_unit
}

pub fn iodine_thyroid_requirement(
    body_weight_kg: f64,
    base_requirement_ug_per_kg: f64,
    pregnancy_factor: f64,
) -> f64 {
    body_weight_kg * base_requirement_ug_per_kg * pregnancy_factor
}

pub fn nutrient_density_score(nutrients: &[f64], daily_values: &[f64], calories: f64) -> f64 {
    let n = nutrients.len().min(daily_values.len());
    if n == 0 || calories <= 0.0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..n {
        sum += (nutrients[i] / daily_values[i].max(1e-30)).min(1.0);
    }
    sum / n as f64 / (calories / 2000.0)
}
