pub fn mazur_two_factor_model(cooling_rate: f64, optimal_rate: f64, width: f64) -> f64 {
    let x = (cooling_rate - optimal_rate) / width;
    (-x * x).exp()
}

pub fn ice_nucleation_rate(temperature: f64, volume: f64, j0: f64, activation_energy: f64) -> f64 {
    volume * j0 * (-activation_energy / (temperature * crate::constants::K_B)).exp()
}

pub fn critical_cooling_rate(cpa_concentration: f64, base_rate: f64, sensitivity: f64) -> f64 {
    base_rate * (-sensitivity * cpa_concentration).exp()
}

pub fn vitrification_probability(cooling_rate: f64, critical_rate: f64) -> f64 {
    1.0 - (-cooling_rate / critical_rate).exp()
}

pub fn cpa_toxicity(concentration: f64, exposure_time: f64, k_tox: f64) -> f64 {
    1.0 - (-k_tox * concentration * exposure_time).exp()
}

pub fn cell_volume_response(v0: f64, osmolarity_ratio: f64, vb: f64) -> f64 {
    vb + (v0 - vb) * osmolarity_ratio
}

pub fn freeze_thaw_survival(
    initial_viability: f64,
    ice_damage: f64,
    osmotic_damage: f64,
    cpa_damage: f64,
) -> f64 {
    initial_viability * (1.0 - ice_damage) * (1.0 - osmotic_damage) * (1.0 - cpa_damage)
}

pub fn intracellular_ice_formation_probability(
    cooling_rate: f64,
    critical_rate: f64,
    n: f64,
) -> f64 {
    let ratio = cooling_rate / critical_rate;
    1.0 - (-ratio.powf(n)).exp()
}

pub fn osmotic_tolerance_limit(v_min: f64, v_max: f64, initial_volume: f64) -> (f64, f64) {
    (v_min / initial_volume, v_max / initial_volume)
}

pub fn kedem_katchalsky_water_flux(
    lp: f64,
    area: f64,
    delta_pi: f64,
    sigma: f64,
    delta_p: f64,
) -> f64 {
    lp * area * (delta_p - sigma * delta_pi)
}

pub fn kedem_katchalsky_solute_flux(
    ps: f64,
    area: f64,
    delta_c: f64,
    sigma: f64,
    jv: f64,
    c_mean: f64,
) -> f64 {
    ps * area * delta_c + (1.0 - sigma) * jv * c_mean
}

pub fn freezing_point_depression(concentration: f64, kf: f64, dissociation_factor: f64) -> f64 {
    kf * concentration * dissociation_factor
}

pub fn hemolysis_fraction(osmolality: f64, half_lysis_osmolality: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (steepness * (osmolality - half_lysis_osmolality)).exp())
}

pub fn stefan_freezing_front(thermal_diffusivity: f64, t: f64, stefan_number: f64) -> f64 {
    2.0 * (thermal_diffusivity * t).sqrt() * stefan_number.sqrt()
}

pub fn supercooling_degree(freezing_point: f64, nucleation_temp: f64) -> f64 {
    freezing_point - nucleation_temp
}

pub fn ice_crystal_growth_rate(
    supercooling: f64,
    diffusivity: f64,
    activation_energy: f64,
    temperature: f64,
) -> f64 {
    diffusivity * supercooling / temperature
        * (-activation_energy / (crate::constants::R_GAS * temperature)).exp()
}

pub fn cpa_loading_protocol_step(
    v: f64,
    lp: f64,
    area: f64,
    osm_in: f64,
    osm_out: f64,
    ps: f64,
    c_in: f64,
    c_out: f64,
    vb: f64,
    dt: f64,
) -> (f64, f64) {
    let jv = lp * area * (osm_in - osm_out);
    let js = ps * area * (c_out - c_in);
    let new_v = (v + jv * dt).max(vb);
    let new_c = (c_in * v + js * dt) / new_v;
    (new_v, new_c)
}

pub fn rewarming_crystallization_risk(warming_rate: f64, critical_warming: f64) -> f64 {
    (-warming_rate / critical_warming).exp()
}

pub fn glass_transition_temperature(cpa_fraction: f64, tg_cpa: f64, tg_water: f64) -> f64 {
    cpa_fraction * tg_cpa + (1.0 - cpa_fraction) * tg_water
}
