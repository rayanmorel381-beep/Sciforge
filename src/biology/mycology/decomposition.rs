pub fn decomposition_rate(k: f64, mass: f64) -> f64 {
    k * mass
}

pub fn remaining_mass(initial: f64, k: f64, t: f64) -> f64 {
    initial * (-k * t).exp()
}

pub fn lignocellulose_decay(
    cellulose: f64,
    lignin: f64,
    k_cellulose: f64,
    k_lignin: f64,
    dt: f64,
) -> (f64, f64) {
    let dc = -k_cellulose * cellulose * (1.0 - lignin / (cellulose + lignin));
    let dl = -k_lignin * lignin;
    ((cellulose + dc * dt).max(0.0), (lignin + dl * dt).max(0.0))
}

pub fn enzyme_mediated_decomposition(substrate: f64, enzyme: f64, vmax: f64, km: f64) -> f64 {
    vmax * enzyme * substrate / (km + substrate)
}

pub fn mycorrhizal_nutrient_exchange(
    plant_carbon: f64,
    fungal_phosphorus: f64,
    exchange_coeff: f64,
) -> (f64, f64) {
    let carbon_to_fungus = exchange_coeff * plant_carbon;
    let phosphorus_to_plant = exchange_coeff * fungal_phosphorus;
    (carbon_to_fungus, phosphorus_to_plant)
}

pub fn saprotrophic_efficiency(carbon_assimilated: f64, carbon_consumed: f64) -> f64 {
    carbon_assimilated / carbon_consumed
}

pub fn humus_formation_rate(
    recalcitrant_fraction: f64,
    decomposition_rate: f64,
    carbon_input: f64,
) -> f64 {
    recalcitrant_fraction * decomposition_rate * carbon_input
}

pub fn nitrogen_mineralization(
    cn_ratio_substrate: f64,
    cn_ratio_microbe: f64,
    carbon_flow: f64,
) -> f64 {
    carbon_flow * (1.0 / cn_ratio_substrate - 1.0 / cn_ratio_microbe)
}

pub fn white_rot_lignin_degradation(
    lignin: f64,
    laccase_activity: f64,
    peroxidase_activity: f64,
    km: f64,
) -> f64 {
    let combined = laccase_activity + peroxidase_activity;
    combined * lignin / (km + lignin)
}

pub fn brown_rot_cellulose_attack(
    cellulose: f64,
    hydroxyl_radical: f64,
    rate_constant: f64,
) -> f64 {
    rate_constant * cellulose * hydroxyl_radical
}

pub fn soft_rot_cavity_formation(
    cellulose: f64,
    moisture: f64,
    enzyme_activity: f64,
    threshold_moisture: f64,
) -> f64 {
    if moisture < threshold_moisture {
        return 0.0;
    }
    enzyme_activity * cellulose * (moisture - threshold_moisture) / moisture
}

pub fn mycorrhizal_carbon_cost(plant_npp: f64, allocation_fraction: f64) -> f64 {
    plant_npp * allocation_fraction
}

pub fn ectomycorrhizal_hyphal_exploration(
    biomass: f64,
    soil_volume: f64,
    exploration_type_factor: f64,
) -> f64 {
    exploration_type_factor * biomass / soil_volume
}

pub fn arbuscular_mycorrhizal_phosphorus_uptake(
    hyphal_length: f64,
    soil_p: f64,
    uptake_rate: f64,
    km: f64,
) -> f64 {
    uptake_rate * hyphal_length * soil_p / (km + soil_p)
}

pub fn wood_decay_mass_loss(
    initial_density: f64,
    fungal_activity: f64,
    moisture_factor: f64,
    temp_factor: f64,
    t: f64,
) -> f64 {
    initial_density * (-fungal_activity * moisture_factor * temp_factor * t).exp()
}

pub fn litter_quality_index(nitrogen_content: f64, lignin_content: f64) -> f64 {
    nitrogen_content / lignin_content.max(1e-30)
}

pub fn carbon_use_efficiency(co2_respired: f64, carbon_assimilated: f64) -> f64 {
    carbon_assimilated / (carbon_assimilated + co2_respired)
}
