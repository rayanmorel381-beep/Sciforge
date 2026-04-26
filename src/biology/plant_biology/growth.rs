pub fn root_growth_logistic(length: f64, max_length: f64, rate: f64, dt: f64) -> f64 {
    length + rate * length * (1.0 - length / max_length) * dt
}

pub fn auxin_gradient(source_concentration: f64, distance: f64, diffusion: f64, decay: f64) -> f64 {
    source_concentration * (-(decay / diffusion).sqrt() * distance).exp()
}

pub fn phototropism_bending_rate(light_differential: f64, sensitivity: f64) -> f64 {
    sensitivity * light_differential
}

pub fn gravitropism_response(angle: f64, sensitivity: f64, dt: f64) -> f64 {
    angle - sensitivity * angle.sin() * dt
}

pub fn leaf_area_index(total_leaf_area: f64, ground_area: f64) -> f64 {
    total_leaf_area / ground_area
}

pub fn beer_lambert_canopy(light_above: f64, k: f64, lai: f64) -> f64 {
    light_above * (-k * lai).exp()
}

pub fn thermal_time(daily_mean_temp: f64, base_temp: f64) -> f64 {
    (daily_mean_temp - base_temp).max(0.0)
}

pub fn water_potential(osmotic: f64, pressure: f64, gravitational: f64) -> f64 {
    osmotic + pressure + gravitational
}

pub fn xylem_flow_rate(pressure_gradient: f64, conductivity: f64, cross_section: f64) -> f64 {
    conductivity * cross_section * pressure_gradient
}

pub fn phloem_transport_munch(source_pressure: f64, sink_pressure: f64, resistance: f64) -> f64 {
    (source_pressure - sink_pressure) / resistance
}

pub fn allometric_biomass(diameter: f64, a: f64, b: f64) -> f64 {
    a * diameter.powf(b)
}

pub fn specific_leaf_area(leaf_area: f64, leaf_dry_mass: f64) -> f64 {
    leaf_area / leaf_dry_mass
}

pub fn relative_growth_rate(biomass_initial: f64, biomass_final: f64, time: f64) -> f64 {
    (biomass_final / biomass_initial).ln() / time
}

pub fn net_assimilation_rate(biomass_change: f64, leaf_area_avg: f64, time: f64) -> f64 {
    biomass_change / (leaf_area_avg * time)
}

pub fn phytochrome_response(red: f64, far_red: f64) -> f64 {
    red / (red + far_red)
}

pub fn vernalization_progress(temp: f64, optimal_temp: f64, range: f64, dt: f64) -> f64 {
    let eff = (-(temp - optimal_temp).powi(2) / (2.0 * range * range)).exp();
    (eff * dt).min(1.0)
}

pub fn photoperiod_response(day_length: f64, critical_length: f64, sensitivity: f64) -> f64 {
    1.0 / (1.0 + (-sensitivity * (day_length - critical_length)).exp())
}

pub fn root_shoot_ratio(root_biomass: f64, shoot_biomass: f64) -> f64 {
    root_biomass / shoot_biomass
}

pub fn canopy_gap_fraction(lai: f64, k: f64) -> f64 {
    (-k * lai).exp()
}

pub fn stem_taper(diameter_base: f64, height_fraction: f64, taper_exponent: f64) -> f64 {
    diameter_base * (1.0 - height_fraction).powf(taper_exponent)
}

pub fn cavitation_vulnerability(pressure: f64, p50: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (slope * (pressure - p50)).exp())
}

pub fn turgor_pressure(osmotic_potential: f64, water_potential: f64) -> f64 {
    (water_potential - osmotic_potential).max(0.0)
}

pub fn gibberellin_stem_elongation(ga_concentration: f64, max_rate: f64, km: f64) -> f64 {
    max_rate * ga_concentration / (km + ga_concentration)
}

pub fn senescence_chlorophyll_loss(chl0: f64, degradation_rate: f64, t: f64) -> f64 {
    chl0 * (-degradation_rate * t).exp()
}

pub fn frost_hardiness(temp: f64, lt50: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (slope * (temp - lt50)).exp())
}
