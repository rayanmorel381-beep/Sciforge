pub fn nutrient_absorption_first_order(dose: f64, ka: f64, t: f64) -> f64 {
    dose * (1.0 - (-ka * t).exp())
}

pub fn gastric_emptying(volume: f64, half_life: f64, t: f64) -> f64 {
    volume * (-std::f64::consts::LN_2 * t / half_life).exp()
}

pub fn glycemic_index_incremental_auc(glucose_values: &[f64], baseline: f64, dt: f64) -> f64 {
    glucose_values
        .iter()
        .map(|&g| (g - baseline).max(0.0) * dt)
        .sum()
}

pub fn protein_digestibility_corrected_amino_acid_score(
    limiting_aa_mg_g: f64,
    reference_mg_g: f64,
    digestibility: f64,
) -> f64 {
    (limiting_aa_mg_g / reference_mg_g * digestibility).min(1.0)
}

pub fn nitrogen_balance(protein_intake_g: f64, urinary_n: f64, fecal_n: f64, sweat_n: f64) -> f64 {
    protein_intake_g / 6.25 - urinary_n - fecal_n - sweat_n
}

pub fn water_requirement_holliday_segar(weight_kg: f64) -> f64 {
    if weight_kg <= 10.0 {
        100.0 * weight_kg
    } else if weight_kg <= 20.0 {
        1000.0 + 50.0 * (weight_kg - 10.0)
    } else {
        1500.0 + 20.0 * (weight_kg - 20.0)
    }
}

pub fn iron_absorption(non_heme_mg: f64, enhancers: f64, inhibitors: f64, heme_mg: f64) -> f64 {
    let non_heme_absorbed = non_heme_mg * 0.05 * (1.0 + enhancers) / (1.0 + inhibitors);
    let heme_absorbed = heme_mg * 0.25;
    non_heme_absorbed + heme_absorbed
}

pub fn calcium_absorption_fraction(intake_mg: f64, vitamin_d_nmol: f64) -> f64 {
    let base = 0.45 - 0.002 * intake_mg;
    let vd_factor = vitamin_d_nmol / (vitamin_d_nmol + 50.0);
    (base * vd_factor).max(0.05)
}

pub fn intestinal_transit_time(fiber_g: f64, fluid_ml: f64, base_time_h: f64) -> f64 {
    base_time_h / ((1.0 + 0.01 * fiber_g) * (1.0 + 0.001 * fluid_ml))
}

pub fn oral_bioavailability(
    fraction_absorbed: f64,
    gut_wall_extraction: f64,
    hepatic_extraction: f64,
) -> f64 {
    fraction_absorbed * (1.0 - gut_wall_extraction) * (1.0 - hepatic_extraction)
}

pub fn michaelis_menten_absorption(vmax: f64, concentration: f64, km: f64) -> f64 {
    vmax * concentration / (km + concentration)
}

pub fn fat_soluble_vitamin_absorption(dose: f64, fat_intake_g: f64, bile_salt_conc: f64) -> f64 {
    let fat_factor = fat_intake_g / (fat_intake_g + 5.0);
    let bile_factor = bile_salt_conc / (bile_salt_conc + 2.0);
    dose * 0.3 * fat_factor * bile_factor
}

pub fn zinc_absorption_fraction(intake_mg: f64, phytate_mg: f64) -> f64 {
    let phytate_ratio = phytate_mg / intake_mg.max(1e-30);
    0.45 / (1.0 + 0.05 * phytate_ratio)
}

pub fn paracellular_absorption(permeability: f64, surface_area: f64, concentration: f64) -> f64 {
    permeability * surface_area * concentration
}

pub fn glucose_transporter_kinetics(glucose: f64, vmax: f64, km: f64, insulin_factor: f64) -> f64 {
    vmax * insulin_factor * glucose / (km + glucose)
}

pub fn amino_acid_absorption_rate(
    concentration: f64,
    vmax: f64,
    km: f64,
    competition_factor: f64,
) -> f64 {
    vmax * concentration / (km * competition_factor + concentration)
}
