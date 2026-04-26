pub fn mycorrhizal_nutrient_transfer(
    root_surface_area: f64,
    hyphal_density: f64,
    nutrient_conc: f64,
    transfer_efficiency: f64,
) -> f64 {
    root_surface_area * hyphal_density * nutrient_conc * transfer_efficiency
}

pub fn mycorrhizal_carbon_allocation(carbon_to_fungus: f64, total_photosynthate: f64) -> f64 {
    carbon_to_fungus / total_photosynthate.max(1e-30)
}

pub fn mycorrhizal_colonization(
    inoculum_potential: f64,
    root_growth_rate: f64,
    susceptibility: f64,
    time: f64,
) -> f64 {
    1.0 - (-inoculum_potential * susceptibility * root_growth_rate * time).exp()
}

pub fn common_mycorrhizal_network_transfer(
    donor_surplus: f64,
    recipient_deficit: f64,
    network_conductance: f64,
) -> f64 {
    network_conductance * (donor_surplus - recipient_deficit).max(0.0)
}

pub fn lichen_photobiont_contribution(
    photobiont_biomass: f64,
    photosynthesis_rate: f64,
    transfer_fraction: f64,
) -> f64 {
    photobiont_biomass * photosynthesis_rate * transfer_fraction
}

pub fn endophyte_benefit(plant_growth_base: f64, endophyte_effect: f64, stress_level: f64) -> f64 {
    plant_growth_base * (1.0 + endophyte_effect * stress_level)
}

pub fn fungal_network_distance(hyphal_growth_rate: f64, branching_angle: f64, time: f64) -> f64 {
    hyphal_growth_rate * time * branching_angle.cos()
}

pub fn truffle_spore_dispersal(
    spore_count: f64,
    wind_speed: f64,
    animal_vectors: f64,
    decay_distance: f64,
    distance: f64,
) -> f64 {
    spore_count * (wind_speed + animal_vectors) * (-distance / decay_distance).exp()
}

pub fn mycobiome_diversity_shannon(abundances: &[f64]) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let mut h = 0.0;
    for &a in abundances {
        if a > 0.0 {
            let p = a / total;
            h -= p * p.ln();
        }
    }
    h
}
