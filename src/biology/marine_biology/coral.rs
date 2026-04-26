pub fn bleaching_thermal_threshold(sst: f64, mmm: f64) -> f64 {
    (sst - mmm - 1.0).max(0.0)
}

pub fn dhw_accumulation(dhw_accumulated: f64, hotspot: f64, dt_weeks: f64) -> f64 {
    if hotspot >= 1.0 {
        dhw_accumulated + hotspot * dt_weeks
    } else {
        (dhw_accumulated - 0.5 * dt_weeks).max(0.0)
    }
}

pub fn calcification_rate(
    omega_aragonite: f64,
    temperature: f64,
    light: f64,
    max_rate: f64,
) -> f64 {
    let omega_factor = (omega_aragonite - 1.0).max(0.0) / omega_aragonite;
    let temp_factor = (-(temperature - 26.0).powi(2) / 50.0).exp();
    let light_factor = light / (light + 200.0);
    max_rate * omega_factor * temp_factor * light_factor
}

pub fn reef_rugosity(surface_distance: f64, linear_distance: f64) -> f64 {
    surface_distance / linear_distance.max(1e-30)
}

pub fn coral_recruitment_rate(
    larval_supply: f64,
    settlement_rate: f64,
    post_settlement_survival: f64,
    available_substrate: f64,
) -> f64 {
    larval_supply * settlement_rate * post_settlement_survival * available_substrate
}

pub fn symbiodinium_density(
    photosynthesis_rate: f64,
    respiration_rate: f64,
    expulsion_rate: f64,
    dt: f64,
    current_density: f64,
) -> f64 {
    let growth = (photosynthesis_rate - respiration_rate) * current_density;
    let loss = expulsion_rate * current_density;
    (current_density + (growth - loss) * dt).max(0.0)
}

pub fn ocean_acidification_saturation(ca_conc: f64, co3_conc: f64, ksp: f64) -> f64 {
    ca_conc * co3_conc / ksp.max(1e-30)
}

pub fn mangrove_carbon_sequestration(biomass: f64, carbon_fraction: f64, growth_rate: f64) -> f64 {
    biomass * carbon_fraction * growth_rate
}

pub fn seagrass_light_attenuation(
    surface_irradiance: f64,
    attenuation_coeff: f64,
    depth: f64,
) -> f64 {
    surface_irradiance * (-attenuation_coeff * depth).exp()
}

pub fn marine_protected_area_spillover(
    biomass_inside: f64,
    biomass_outside: f64,
    perimeter_area_ratio: f64,
    diffusion: f64,
) -> f64 {
    diffusion * perimeter_area_ratio * (biomass_inside - biomass_outside)
}
