use crate::constants::{
    DENSITY_ALTITUDE_SCALE, DRY_ADIABATIC_LAPSE_RATE, EARTH_GRAVITY, ISA_LAPSE_RATE_PER_M,
    ISA_SEA_LEVEL_TEMP_C, MAGNUS_A, MAGNUS_B, MIXING_RATIO_FACTOR, R_GAS,
    SATURATION_VAPOR_PRESSURE_0C, SPECIFIC_GAS_CONSTANT_DRY_AIR, SPECIFIC_GAS_CONSTANT_WATER_VAPOR,
    SPECIFIC_HEAT_DRY_AIR, VIRTUAL_TEMP_FACTOR,
};

pub fn barometric_formula(p0: f64, m: f64, g: f64, h: f64, t: f64) -> f64 {
    p0 * (-m * g * h / (R_GAS * t)).exp()
}

pub fn scale_height(t: f64, m: f64, g: f64) -> f64 {
    R_GAS * t / (m * g)
}

pub fn lapse_rate_dry() -> f64 {
    EARTH_GRAVITY / SPECIFIC_HEAT_DRY_AIR * 1e3
}

pub fn lapse_rate_moist(t: f64, l_v: f64, r_s: f64) -> f64 {
    EARTH_GRAVITY * (1.0 + l_v * r_s / (SPECIFIC_GAS_CONSTANT_DRY_AIR * t))
        / (1.0
            + l_v * l_v * r_s / (SPECIFIC_HEAT_DRY_AIR * SPECIFIC_GAS_CONSTANT_WATER_VAPOR * t * t))
}

pub fn potential_temperature(t: f64, p: f64, p0: f64) -> f64 {
    t * (p0 / p).powf(SPECIFIC_GAS_CONSTANT_DRY_AIR / SPECIFIC_HEAT_DRY_AIR)
}

pub fn virtual_temperature(t: f64, r: f64) -> f64 {
    t * (1.0 + VIRTUAL_TEMP_FACTOR * r)
}

pub fn mixing_ratio(e: f64, p: f64) -> f64 {
    MIXING_RATIO_FACTOR * e / (p - e)
}

pub fn saturation_vapor_pressure(t_celsius: f64) -> f64 {
    SATURATION_VAPOR_PRESSURE_0C * (MAGNUS_A * t_celsius / (t_celsius + MAGNUS_B)).exp()
}

pub fn relative_humidity(e: f64, es: f64) -> f64 {
    e / es * 100.0
}

pub fn dew_point(t: f64, rh: f64) -> f64 {
    let gamma = (MAGNUS_A * t / (MAGNUS_B + t)).exp() * rh / 100.0;
    MAGNUS_B * gamma.ln() / (MAGNUS_A - gamma.ln())
}

pub fn density_altitude(pressure_altitude: f64, temperature_c: f64) -> f64 {
    pressure_altitude
        + DENSITY_ALTITUDE_SCALE
            * (temperature_c - (ISA_SEA_LEVEL_TEMP_C - ISA_LAPSE_RATE_PER_M * pressure_altitude))
}

pub fn brunt_vaisala_frequency(g: f64, theta: f64, dtheta_dz: f64) -> f64 {
    (g / theta * dtheta_dz).sqrt()
}

pub fn rayleigh_phase(cos_theta: f64) -> f64 {
    3.0 / (16.0 * std::f64::consts::PI) * (1.0 + cos_theta.powi(2))
}

pub fn mie_phase(cos_theta: f64, g: f64) -> f64 {
    let g2 = g * g;
    let denom = (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5);
    3.0 * (1.0 - g2) * (1.0 + cos_theta.powi(2)) / (8.0 * std::f64::consts::PI * (2.0 + g2) * denom)
}

pub fn lifted_condensation_level(t_surface: f64, dew_point: f64) -> f64 {
    (t_surface - dew_point) / DRY_ADIABATIC_LAPSE_RATE
}

pub fn dry_adiabatic_temperature(t_surface: f64, altitude: f64) -> f64 {
    t_surface - DRY_ADIABATIC_LAPSE_RATE * altitude
}

pub fn convective_available_potential_energy(t_parcel: &[f64], t_env: &[f64], dz: f64) -> f64 {
    t_parcel
        .iter()
        .zip(t_env.iter())
        .map(|(&tp, &te)| {
            let buoyancy = EARTH_GRAVITY * (tp - te) / te;
            if buoyancy > 0.0 { buoyancy * dz } else { 0.0 }
        })
        .sum()
}

pub fn convective_inhibition(t_parcel: &[f64], t_env: &[f64], dz: f64) -> f64 {
    t_parcel
        .iter()
        .zip(t_env.iter())
        .map(|(&tp, &te)| {
            let buoyancy = EARTH_GRAVITY * (tp - te) / te;
            if buoyancy < 0.0 { buoyancy * dz } else { 0.0 }
        })
        .sum()
}
