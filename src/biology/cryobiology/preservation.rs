pub fn storage_decay_arrhenius(a: f64, ea: f64, temperature_k: f64) -> f64 {
    a * (-ea / (crate::constants::R_GAS * temperature_k)).exp()
}

pub fn shelf_life(initial_viability: f64, threshold: f64, decay_rate: f64) -> f64 {
    if decay_rate <= 0.0 {
        return f64::INFINITY;
    }
    (initial_viability / threshold).ln() / decay_rate
}

pub fn recrystallization_rate(temperature: f64, activation_energy: f64, pre_factor: f64) -> f64 {
    pre_factor * (-activation_energy / (crate::constants::R_GAS * temperature)).exp()
}

pub fn warming_rate_survival(warming_rate: f64, optimal_warming: f64, sigma: f64) -> f64 {
    let x = (warming_rate - optimal_warming) / sigma;
    (-0.5 * x * x).exp()
}

pub fn devitrification_probability(warming_rate: f64, critical_warming_rate: f64) -> f64 {
    (-warming_rate / critical_warming_rate).exp()
}

pub fn cpa_permeation(
    permeability: f64,
    surface_area: f64,
    concentration_out: f64,
    concentration_in: f64,
) -> f64 {
    permeability * surface_area * (concentration_out - concentration_in)
}

pub fn two_parameter_model_volume(
    volume0: f64,
    lp: f64,
    surface_area: f64,
    osm_out: f64,
    osm_in: f64,
    dt: f64,
) -> f64 {
    let dv = lp * surface_area * (osm_in - osm_out);
    (volume0 + dv * dt).max(0.01)
}

pub fn cooling_rate_survival(cooling_rate: f64, optimal: f64, sigma: f64) -> f64 {
    let x = (cooling_rate - optimal) / sigma;
    (-0.5 * x * x).exp()
}

pub fn ice_nucleation_probability(temperature: f64, volume: f64, j0: f64, ea: f64) -> f64 {
    let j = j0 * (-ea / (crate::constants::R_GAS * temperature)).exp();
    1.0 - (-j * volume).exp()
}

pub fn lyophilization_primary_drying_rate(heat_input: f64, sublimation_enthalpy: f64) -> f64 {
    heat_input / sublimation_enthalpy
}

pub fn lyophilization_collapse_temperature(tg_prime: f64, offset: f64) -> f64 {
    tg_prime + offset
}

pub fn trehalose_protection(trehalose_conc: f64, k_protect: f64, max_protection: f64) -> f64 {
    max_protection * trehalose_conc / (k_protect + trehalose_conc)
}

pub fn thawing_temperature_profile(t_initial: f64, t_bath: f64, k: f64, time: f64) -> f64 {
    t_bath + (t_initial - t_bath) * (-k * time).exp()
}

pub fn post_thaw_recovery_kinetics(plateau: f64, recovery_rate: f64, t: f64) -> f64 {
    plateau * (1.0 - (-recovery_rate * t).exp())
}

pub fn controlled_rate_freezer_program(target_rate: f64, current_temp: f64, dt: f64) -> f64 {
    current_temp - target_rate * dt
}

pub fn thermal_seed_temperature(sample_temp: f64, seed_offset: f64) -> f64 {
    sample_temp - seed_offset
}

pub fn isochoric_preservation_pressure(
    temperature: f64,
    reference_temp: f64,
    bulk_modulus: f64,
    expansion_coeff: f64,
) -> f64 {
    bulk_modulus * expansion_coeff * (reference_temp - temperature)
}

pub fn q10_temperature_coefficient(rate_t2: f64, rate_t1: f64, t2: f64, t1: f64) -> f64 {
    (rate_t2 / rate_t1).powf(10.0 / (t2 - t1))
}

pub fn wlf_viscosity_shift(c1: f64, c2: f64, temperature: f64, tg: f64) -> f64 {
    -c1 * (temperature - tg) / (c2 + temperature - tg)
}
