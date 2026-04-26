pub fn hyphal_growth_rate(tip_extension: f64, branching_rate: f64, tips: f64) -> f64 {
    tip_extension * tips * (1.0 + branching_rate)
}

pub fn colony_radial_growth(r0: f64, rate: f64, t: f64) -> f64 {
    r0 + rate * t
}

pub fn spore_germination_probability(
    water_activity: f64,
    temperature: f64,
    aw_min: f64,
    t_min: f64,
    t_max: f64,
) -> f64 {
    if water_activity < aw_min || temperature < t_min || temperature > t_max {
        return 0.0;
    }
    let aw_factor = (water_activity - aw_min) / (1.0 - aw_min);
    let t_factor = (temperature - t_min) * (t_max - temperature) / ((t_max - t_min) / 2.0).powi(2);
    (aw_factor * t_factor).clamp(0.0, 1.0)
}

pub fn mycelial_network_transport(
    concentration_source: f64,
    concentration_sink: f64,
    conductance: f64,
) -> f64 {
    conductance * (concentration_source - concentration_sink)
}

pub fn chitin_content(dry_mass: f64, chitin_fraction: f64) -> f64 {
    dry_mass * chitin_fraction
}

pub fn fungal_biomass_from_ergosterol(ergosterol_ug: f64, conversion_factor: f64) -> f64 {
    ergosterol_ug * conversion_factor
}

pub fn substrate_colonization_speed(growth_rate: f64, nutrient_availability: f64, km: f64) -> f64 {
    growth_rate * nutrient_availability / (km + nutrient_availability)
}

pub fn fairy_ring_radius(initial_radius: f64, expansion_rate: f64, t: f64) -> f64 {
    initial_radius + expansion_rate * t
}

pub fn spore_dispersal_distance(
    wind_speed: f64,
    release_height: f64,
    terminal_velocity: f64,
) -> f64 {
    wind_speed * release_height / terminal_velocity
}

pub fn yeast_budding_rate(
    nutrient: f64,
    temperature: f64,
    optimal_temp: f64,
    temp_width: f64,
) -> f64 {
    let temp_factor =
        (-(temperature - optimal_temp).powi(2) / (2.0 * temp_width * temp_width)).exp();
    nutrient * temp_factor / (1.0 + nutrient)
}

pub fn mycelial_biomass_logistic(biomass: f64, max_biomass: f64, mu_max: f64, dt: f64) -> f64 {
    let growth = mu_max * biomass * (1.0 - biomass / max_biomass);
    (biomass + growth * dt).max(0.0)
}

pub fn branching_frequency(hyphal_length: f64, branch_count: f64) -> f64 {
    branch_count / hyphal_length.max(1e-30)
}

pub fn hyphal_tip_vesicle_supply(
    vesicle_production: f64,
    distance_to_tip: f64,
    diffusion: f64,
) -> f64 {
    vesicle_production * (-distance_to_tip.powi(2) / (2.0 * diffusion)).exp()
}

pub fn conidiation_rate(nutrient_depletion: f64, light_signal: f64, threshold: f64) -> f64 {
    let stress = (1.0 - nutrient_depletion).max(0.0);
    (stress + light_signal) / (threshold + stress + light_signal)
}

pub fn rhizomorph_transport_rate(
    pressure_gradient: f64,
    conductance: f64,
    cross_section: f64,
) -> f64 {
    conductance * cross_section * pressure_gradient
}

pub fn lichenization_benefit(
    algal_photosynthate: f64,
    fungal_protection: f64,
    exchange_rate: f64,
) -> f64 {
    exchange_rate * algal_photosynthate * fungal_protection
}

pub fn spore_survival_uv(initial_viability: f64, uv_dose: f64, sensitivity: f64) -> f64 {
    initial_viability * (-sensitivity * uv_dose).exp()
}

pub fn monod_fungal_growth(mu_max: f64, substrate: f64, ks: f64) -> f64 {
    mu_max * substrate / (ks + substrate)
}

pub fn biofilm_formation_rate(
    cell_density: f64,
    surface_affinity: f64,
    quorum_signal: f64,
    threshold: f64,
) -> f64 {
    surface_affinity * cell_density * quorum_signal / (threshold + quorum_signal)
}
