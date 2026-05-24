use crate::constants::{
    ATM_TO_PASCAL, BAR_TO_PASCAL, CALORIE_TO_JOULE, K_B, KEV_TO_KELVIN, KWH_TO_J, R_GAS,
};

pub fn ideal_gas_pressure(n_moles: f64, temperature: f64, volume: f64) -> f64 {
    n_moles * R_GAS * temperature / volume
}

pub fn ideal_gas_volume(n_moles: f64, temperature: f64, pressure: f64) -> f64 {
    n_moles * R_GAS * temperature / pressure
}

pub fn ideal_gas_temperature(pressure: f64, volume: f64, n_moles: f64) -> f64 {
    pressure * volume / (n_moles * R_GAS)
}

pub fn van_der_waals_pressure(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) -> f64 {
    let vm = volume / n_moles;
    (R_GAS * temperature / (vm - b)) - a / (vm * vm)
}

pub fn redlich_kwong_pressure(n_moles: f64, temperature: f64, volume: f64, a: f64, b: f64) -> f64 {
    let vm = volume / n_moles;
    R_GAS * temperature / (vm - b) - a / (temperature.sqrt() * vm * (vm + b))
}

pub fn virial_eos(pressure: f64, temperature: f64, b2: f64, b3: f64) -> f64 {
    let rho_approx = pressure / (R_GAS * temperature);
    let rho = rho_approx * (1.0 + b2 * rho_approx + b3 * rho_approx * rho_approx);
    1.0 / rho
}

pub fn compressibility_factor(pressure: f64, volume: f64, n_moles: f64, temperature: f64) -> f64 {
    pressure * volume / (n_moles * R_GAS * temperature)
}

pub fn internal_energy_ideal(n_moles: f64, cv: f64, temperature: f64) -> f64 {
    n_moles * cv * temperature
}

pub fn enthalpy_ideal(n_moles: f64, cp: f64, temperature: f64) -> f64 {
    n_moles * cp * temperature
}

pub fn entropy_ideal_gas(n_moles: f64, cv: f64, t1: f64, t2: f64, v1: f64, v2: f64) -> f64 {
    n_moles * (cv * (t2 / t1).ln() + R_GAS * (v2 / v1).ln())
}

pub fn gibbs_free_energy(enthalpy: f64, temperature: f64, entropy: f64) -> f64 {
    enthalpy - temperature * entropy
}

pub fn helmholtz_free_energy(internal_energy: f64, temperature: f64, entropy: f64) -> f64 {
    internal_energy - temperature * entropy
}

pub fn chemical_potential_ideal_gas(mu0: f64, temperature: f64, pressure: f64, p0: f64) -> f64 {
    mu0 + R_GAS * temperature * (pressure / p0).ln()
}

pub fn clausius_clapeyron(p1: f64, t1: f64, t2: f64, delta_h_vap: f64) -> f64 {
    p1 * (delta_h_vap / R_GAS * (1.0 / t1 - 1.0 / t2)).exp()
}

pub fn heat_capacity_ratio(cp: f64, cv: f64) -> f64 {
    cp / cv
}

pub fn speed_of_sound_ideal(gamma: f64, temperature: f64, molar_mass: f64) -> f64 {
    (gamma * R_GAS * temperature / molar_mass).sqrt()
}

pub fn maxwell_speed_distribution(v: f64, mass: f64, temperature: f64) -> f64 {
    let a = mass / (2.0 * K_B * temperature);
    4.0 * std::f64::consts::PI * (a / std::f64::consts::PI).powf(1.5) * v * v * (-a * v * v).exp()
}

pub fn most_probable_speed(mass: f64, temperature: f64) -> f64 {
    (2.0 * K_B * temperature / mass).sqrt()
}

pub fn mean_speed(mass: f64, temperature: f64) -> f64 {
    (8.0 * K_B * temperature / (std::f64::consts::PI * mass)).sqrt()
}

pub fn rms_speed(mass: f64, temperature: f64) -> f64 {
    (3.0 * K_B * temperature / mass).sqrt()
}

pub fn mean_free_path(number_density: f64, cross_section: f64) -> f64 {
    1.0 / (2.0_f64.sqrt() * number_density * cross_section)
}

pub fn pressure_atm_to_pascal(p_atm: f64) -> f64 {
    p_atm * ATM_TO_PASCAL
}

pub fn pressure_pascal_to_atm(p_pa: f64) -> f64 {
    p_pa / ATM_TO_PASCAL
}

pub fn pressure_bar_to_pascal(p_bar: f64) -> f64 {
    p_bar * BAR_TO_PASCAL
}

pub fn energy_calories_to_joules(cal: f64) -> f64 {
    cal * CALORIE_TO_JOULE
}

pub fn energy_joules_to_calories(j: f64) -> f64 {
    j / CALORIE_TO_JOULE
}

pub fn energy_kwh_to_joules(kwh: f64) -> f64 {
    kwh * KWH_TO_J
}

pub fn plasma_temperature_kev_to_kelvin(t_kev: f64) -> f64 {
    t_kev * KEV_TO_KELVIN
}

pub fn ideal_gas_pressure_atm(n_moles: f64, temperature: f64, volume_liters: f64) -> f64 {
    n_moles * R_GAS * temperature / (volume_liters * 1e-3) / ATM_TO_PASCAL
}
