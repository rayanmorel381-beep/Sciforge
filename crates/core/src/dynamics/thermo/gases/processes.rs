use super::GasState;
use crate::moleculars::Gas;
use sciforge_hub::prelude::meteorology::atmosphere as sf_atmo;
use sciforge_hub::prelude::physics::thermodynamics as sf_thermo;

pub fn isothermal_compression<'a>(state: &GasState<'a>, p2_pa: f64) -> GasState<'a> {
    GasState::new(state.gas, p2_pa, state.temperature_k)
}

pub fn adiabatic_compression<'a>(state: &GasState<'a>, p2_pa: f64) -> GasState<'a> {
    let gamma = state.gas.gamma();
    let t2 = state.temperature_k * (p2_pa / state.pressure_pa).powf((gamma - 1.0) / gamma);
    GasState::new(state.gas, p2_pa, t2)
}

pub fn isobaric_heating<'a>(state: &GasState<'a>, t2_k: f64) -> GasState<'a> {
    GasState::new(state.gas, state.pressure_pa, t2_k)
}

pub fn isochoric_heating<'a>(state: &GasState<'a>, t2_k: f64) -> GasState<'a> {
    let p2 = state.pressure_pa * t2_k / state.temperature_k;
    GasState::new(state.gas, p2, t2_k)
}

pub fn polytropic_compression<'a>(state: &GasState<'a>, p2_pa: f64, n: f64) -> GasState<'a> {
    let t2 = state.temperature_k * (p2_pa / state.pressure_pa).powf((n - 1.0) / n);
    GasState::new(state.gas, p2_pa, t2)
}

pub fn isothermal_work_j_kg(state: &GasState, p2_pa: f64) -> f64 {
    let n_moles_per_kg = 1.0 / state.gas.molar_mass_kg_per_mol;
    let v1 = 1.0;
    let v2 = state.pressure_pa / p2_pa;
    -sf_thermo::isothermal_work(n_moles_per_kg, state.temperature_k, v1, v2)
}

pub fn isobaric_work_j_kg(state: &GasState, t2_k: f64) -> f64 {
    let r = state.gas.specific_gas_constant_j_kgk();
    let v1 = r * state.temperature_k / state.pressure_pa;
    let v2 = r * t2_k / state.pressure_pa;
    sf_thermo::isobaric_work(state.pressure_pa, v1, v2)
}

pub fn adiabatic_work_j_kg(state: &GasState, p2_pa: f64) -> f64 {
    let gamma = state.gas.gamma();
    let t2 = state.temperature_k * (p2_pa / state.pressure_pa).powf((gamma - 1.0) / gamma);
    let n_moles_per_kg = 1.0 / state.gas.molar_mass_kg_per_mol;
    let cv_molar_j_molk = state.gas.cv_j_kgk_ref * state.gas.molar_mass_kg_per_mol;
    sf_thermo::adiabatic_work(n_moles_per_kg, cv_molar_j_molk, state.temperature_k, t2)
}

pub fn polytropic_work_j_kg(state: &GasState, p2_pa: f64, n: f64) -> f64 {
    let r = state.gas.specific_gas_constant_j_kgk();
    let t2 = state.temperature_k * (p2_pa / state.pressure_pa).powf((n - 1.0) / n);
    r * (state.temperature_k - t2) / (n - 1.0)
}

pub fn isentropic_efficiency(actual_work_j_kg: f64, ideal_work_j_kg: f64) -> f64 {
    ideal_work_j_kg / actual_work_j_kg
}

pub fn stagnation_temperature_k(state: &GasState, velocity_m_s: f64) -> f64 {
    state.temperature_k + velocity_m_s * velocity_m_s / (2.0 * state.gas.cp_j_kgk_ref)
}

pub fn stagnation_pressure_pa(state: &GasState, velocity_m_s: f64) -> f64 {
    let gamma = state.gas.gamma();
    let mach = state.mach_number(velocity_m_s);
    state.pressure_pa * (1.0 + (gamma - 1.0) / 2.0 * mach * mach).powf(gamma / (gamma - 1.0))
}

pub fn choking_mass_flow_kg_s(gas: &Gas, pressure_pa: f64, temperature_k: f64, area_m2: f64) -> f64 {
    let gamma = gas.gamma();
    let r = gas.specific_gas_constant_j_kgk();
    let rho = pressure_pa / (r * temperature_k);
    let a_sound = gas.speed_of_sound_m_s(temperature_k);
    let factor = (gamma * (2.0 / (gamma + 1.0)).powf((gamma + 1.0) / (gamma - 1.0))).sqrt();
    rho * a_sound * area_m2 * factor / gamma.sqrt()
}

pub fn potential_temperature_k(temperature_k: f64, pressure_pa: f64, reference_pressure_pa: f64) -> f64 {
    sf_atmo::potential_temperature(temperature_k, pressure_pa, reference_pressure_pa)
}

pub fn dry_adiabatic_temperature_k(surface_temperature_k: f64, altitude_m: f64) -> f64 {
    sf_atmo::dry_adiabatic_temperature(surface_temperature_k, altitude_m)
}
