use super::LiquidState;
use sciforge_hub::prelude::constants::EARTH_GRAVITY;

pub fn cavitation_number(
    state: &LiquidState,
    reference_pressure_pa: f64,
    velocity_m_s: f64,
) -> f64 {
    let rho = state.liquid.density_kg_m3_ref;
    let pv = state.liquid.vapor_pressure_pa_ref;
    (reference_pressure_pa - pv) / (0.5 * rho * velocity_m_s * velocity_m_s)
}

pub fn is_cavitating(
    state: &LiquidState,
    reference_pressure_pa: f64,
    velocity_m_s: f64,
) -> bool {
    cavitation_number(state, reference_pressure_pa, velocity_m_s) < 1.0
}

pub fn npsh_available_m(
    state: &LiquidState,
    absolute_pressure_pa: f64,
    velocity_m_s: f64,
    elevation_m: f64,
) -> f64 {
    let rho = state.liquid.density_kg_m3_ref;
    let pv = state.liquid.vapor_pressure_pa_ref;
    (absolute_pressure_pa - pv) / (rho * EARTH_GRAVITY)
        + velocity_m_s * velocity_m_s / (2.0 * EARTH_GRAVITY)
        + elevation_m
}

pub fn critical_velocity_m_s(state: &LiquidState, reference_pressure_pa: f64) -> f64 {
    let rho = state.liquid.density_kg_m3_ref;
    let pv = state.liquid.vapor_pressure_pa_ref;
    ((2.0 * (reference_pressure_pa - pv)) / rho).sqrt()
}

pub fn thoma_cavitation_coefficient(npsha_m: f64, npshr_m: f64, head_m: f64) -> f64 {
    (npsha_m - npshr_m) / head_m
}
