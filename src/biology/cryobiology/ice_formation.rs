pub fn nucleation_temperature(solution_concentration: f64, cooling_rate: f64) -> f64 {
    -2.0 - 0.5 * cooling_rate - 0.1 * solution_concentration
}

pub fn ice_growth_rate(supercooling: f64, diffusion_coeff: f64, latent_heat: f64) -> f64 {
    diffusion_coeff * supercooling.powi(2) / latent_heat
}

pub fn intracellular_ice_probability(cooling_rate: f64, critical_rate: f64) -> f64 {
    1.0 / (1.0 + (critical_rate / cooling_rate).powi(2))
}

pub fn ostwald_recrystallization_rate(temperature: f64, activation_energy: f64) -> f64 {
    use crate::constants::K_B;
    let t_abs = temperature + 273.15;
    1e10 * (-activation_energy / (K_B * t_abs)).exp()
}

pub fn anti_freeze_protein_thermal_hysteresis(concentration: f64, k_th: f64, n: f64) -> f64 {
    k_th * concentration.powf(n)
}

pub fn cryoprotectant_toxicity(
    concentration: f64,
    temperature: f64,
    exposure_time: f64,
    k_tox: f64,
) -> f64 {
    k_tox * concentration * exposure_time * (temperature + 273.15) / 310.0
}

pub fn dehydration_during_freezing(
    initial_water: f64,
    osmotic_coefficient: f64,
    ice_fraction: f64,
) -> f64 {
    initial_water * (1.0 - osmotic_coefficient * ice_fraction)
}

pub fn vitrification_temperature(
    water_fraction: f64,
    tg_pure_solute: f64,
    tg_water: f64,
    k_gt: f64,
) -> f64 {
    (water_fraction * tg_water + k_gt * (1.0 - water_fraction) * tg_pure_solute)
        / (water_fraction + k_gt * (1.0 - water_fraction))
}
