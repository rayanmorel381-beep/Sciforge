pub fn saprotrophic_decomposition(
    substrate_mass: f64,
    enzyme_activity: f64,
    moisture: f64,
    temperature: f64,
    optimal_temp: f64,
) -> f64 {
    let temp_factor = (-(temperature - optimal_temp).powi(2) / 100.0).exp();
    let moisture_factor = moisture / (0.3 + moisture);
    enzyme_activity * substrate_mass * temp_factor * moisture_factor
}

pub fn wood_decay_rate(
    lignin_fraction: f64,
    cellulose_fraction: f64,
    fungal_type_lignin_pref: f64,
) -> f64 {
    fungal_type_lignin_pref * lignin_fraction + (1.0 - fungal_type_lignin_pref) * cellulose_fraction
}

pub fn fungal_succession_priority(colonization_order: usize, competitive_ability: f64) -> f64 {
    competitive_ability / colonization_order.max(1) as f64
}

pub fn spore_germination_rate(
    moisture: f64,
    temperature: f64,
    temp_min: f64,
    temp_max: f64,
    temp_opt: f64,
) -> f64 {
    if temperature < temp_min || temperature > temp_max {
        return 0.0;
    }
    let temp_factor =
        (-(temperature - temp_opt).powi(2) / ((temp_max - temp_min).powi(2) / 4.0)).exp();
    let moisture_factor = (moisture / 0.9).min(1.0);
    temp_factor * moisture_factor
}

pub fn fairy_ring_expansion(ring_radius: f64, growth_rate: f64, nutrient_depletion: f64) -> f64 {
    growth_rate * (1.0 - nutrient_depletion) / ring_radius.max(1e-30)
}

pub fn ergosterol_biomass_estimate(ergosterol_conc: f64, conversion_factor: f64) -> f64 {
    ergosterol_conc * conversion_factor
}

pub fn fungal_carbon_mineralization(biomass: f64, cue: f64, substrate_consumed: f64) -> f64 {
    biomass * substrate_consumed * (1.0 - cue)
}

pub fn mycelial_network_resilience(connections: usize, nodes: usize, redundancy: f64) -> f64 {
    let connectivity =
        connections as f64 / (nodes.max(1) as f64 * (nodes.max(1) as f64 - 1.0) / 2.0).max(1e-30);
    connectivity * (1.0 + redundancy)
}
