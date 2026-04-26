pub fn net_primary_productivity(gpp: f64, autotrophic_respiration: f64) -> f64 {
    gpp - autotrophic_respiration
}

pub fn net_ecosystem_productivity(npp: f64, heterotrophic_respiration: f64) -> f64 {
    npp - heterotrophic_respiration
}

pub fn carbon_use_efficiency(npp: f64, gpp: f64) -> f64 {
    npp / gpp.max(1e-30)
}

pub fn nitrogen_mineralization(
    organic_n: f64,
    microbial_activity: f64,
    temperature_factor: f64,
) -> f64 {
    organic_n * microbial_activity * temperature_factor
}

pub fn nutrient_use_efficiency(biomass_produced: f64, nutrient_absorbed: f64) -> f64 {
    biomass_produced / nutrient_absorbed.max(1e-30)
}

pub fn liebig_minimum(nutrients: &[f64], requirements: &[f64]) -> f64 {
    let n = nutrients.len().min(requirements.len());
    if n == 0 {
        return 0.0;
    }
    let mut min_ratio = f64::INFINITY;
    for i in 0..n {
        let ratio = nutrients[i] / requirements[i].max(1e-30);
        if ratio < min_ratio {
            min_ratio = ratio;
        }
    }
    min_ratio
}

pub fn decomposition_rate(initial_mass: f64, k: f64, t: f64) -> f64 {
    initial_mass * (-k * t).exp()
}

pub fn soil_respiration(temperature: f64, moisture: f64, q10: f64, r_ref: f64) -> f64 {
    let temp_factor = q10.powf((temperature - 25.0) / 10.0);
    let moisture_factor = moisture / (0.5 + moisture);
    r_ref * temp_factor * moisture_factor
}

pub fn evapotranspiration_penman_monteith(
    net_radiation: f64,
    soil_heat_flux: f64,
    air_temp: f64,
    vpd: f64,
    wind_speed: f64,
    surface_resistance: f64,
) -> f64 {
    let gamma = 0.066;
    let delta = 4098.0 * (0.6108 * (17.27 * air_temp / (air_temp + 237.3)).exp())
        / (air_temp + 237.3).powi(2);
    let rho_cp = 1.013;
    let ra = 208.0 / wind_speed.max(0.1);
    let num = delta * (net_radiation - soil_heat_flux) + rho_cp * vpd / ra;
    let den = delta + gamma * (1.0 + surface_resistance / ra);
    num / den.max(1e-30)
}

pub fn water_use_efficiency(carbon_assimilated: f64, water_transpired: f64) -> f64 {
    carbon_assimilated / water_transpired.max(1e-30)
}

pub fn litter_bag_decomposition(initial_mass: f64, remaining_mass: f64, time: f64) -> f64 {
    if time <= 0.0 {
        return 0.0;
    }
    -(remaining_mass / initial_mass.max(1e-30)).ln() / time
}
