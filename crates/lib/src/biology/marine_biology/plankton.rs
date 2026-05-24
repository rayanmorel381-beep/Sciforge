pub fn phytoplankton_growth(mu_max: f64, nutrient: f64, ks: f64, light: f64, ki: f64) -> f64 {
    let nutrient_limitation = nutrient / (ks + nutrient);
    let light_limitation = light / (ki + light);
    mu_max * nutrient_limitation * light_limitation
}

pub fn bloom_critical_depth(
    surface_irradiance: f64,
    compensation_irradiance: f64,
    attenuation: f64,
) -> f64 {
    if attenuation <= 0.0 {
        return f64::INFINITY;
    }
    (surface_irradiance / compensation_irradiance.max(1e-30)).ln() / attenuation
}

pub fn sverdrup_critical_depth(
    avg_irradiance: f64,
    compensation_irradiance: f64,
    mixed_layer_depth: f64,
    attenuation: f64,
) -> f64 {
    let integrated =
        avg_irradiance / attenuation * (1.0 - (-attenuation * mixed_layer_depth).exp());
    integrated / compensation_irradiance.max(1e-30)
}

pub fn nutrient_phytoplankton_zooplankton_step(
    n: f64,
    p: f64,
    z: f64,
    dt: f64,
    mu: f64,
    ks: f64,
    grazing: f64,
    kp: f64,
    mortality_z: f64,
    recycling: f64,
) -> (f64, f64, f64) {
    let uptake = mu * n / (ks + n) * p;
    let graze = grazing * p * p / (kp * kp + p * p) * z;
    let dn = -uptake + recycling * z * mortality_z;
    let dp = uptake - graze;
    let dz = 0.3 * graze - mortality_z * z;
    (
        (n + dn * dt).max(0.0),
        (p + dp * dt).max(0.0),
        (z + dz * dt).max(0.0),
    )
}

pub fn chlorophyll_a_from_biomass(biomass: f64, carbon_to_chl_ratio: f64) -> f64 {
    biomass / carbon_to_chl_ratio.max(1e-30)
}

pub fn zooplankton_diel_migration_depth(
    daytime_depth: f64,
    nighttime_depth: f64,
    time_hours: f64,
) -> f64 {
    let pi = std::f64::consts::PI;
    let phase = (2.0 * pi * time_hours / 24.0).cos();
    (daytime_depth + nighttime_depth) / 2.0 + (daytime_depth - nighttime_depth) / 2.0 * phase
}

pub fn harmful_algal_bloom_toxin(
    cell_density: f64,
    toxin_per_cell: f64,
    decay_rate: f64,
    t: f64,
) -> f64 {
    cell_density * toxin_per_cell * (1.0 - (-decay_rate * t).exp())
}

pub fn redfield_ratio_deviation(c: f64, n: f64, p: f64) -> (f64, f64) {
    let cn_ratio = c / n.max(1e-30);
    let np_ratio = n / p.max(1e-30);
    (cn_ratio / 6.625, np_ratio / 16.0)
}

pub fn spring_bloom_timing(mixed_layer_depth: f64, critical_depth: f64) -> bool {
    mixed_layer_depth < critical_depth
}

pub fn export_production(primary_production: f64, f_ratio: f64) -> f64 {
    primary_production * f_ratio
}
