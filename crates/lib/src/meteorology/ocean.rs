use crate::constants::{
    EARTH_GRAVITY, G, HALINE_CONTRACTION_0, HALINE_CONTRACTION_1, JONSWAP_ALPHA_COEFF,
    JONSWAP_ALPHA_EXPONENT, JONSWAP_FREQ_COEFF, JONSWAP_GAMMA, JONSWAP_SIGMA_A, JONSWAP_SIGMA_B,
    PHILLIPS_ALPHA, PHILLIPS_BETA, R_GAS, SEAWATER_RHO_0, SEAWATER_RHO_B_T1, SEAWATER_RHO_B_T2,
    SEAWATER_RHO_B0, SEAWATER_RHO_C0, SEAWATER_RHO_S_T1, SEAWATER_RHO_S_T2, SEAWATER_RHO_S_T3,
    SEAWATER_RHO_S_T4, SEAWATER_RHO_S0, SEAWATER_RHO_T1, SEAWATER_RHO_T2, SEAWATER_RHO_T3,
    SEAWATER_RHO_T4, SEAWATER_RHO_T5, SOUND_SPEED_BASE, SOUND_SPEED_D1, SOUND_SPEED_D2,
    SOUND_SPEED_S1, SOUND_SPEED_T1, SOUND_SPEED_T2, SOUND_SPEED_T3, SOUND_SPEED_TD, SOUND_SPEED_TS,
    THERMAL_EXPANSION_0, THERMAL_EXPANSION_1, WAVE_HEIGHT_COEFF, WAVE_HEIGHT_EXPONENT,
    WAVE_PERIOD_COEFF,
};

pub fn deep_water_phase_speed(g_local: f64, wavelength: f64) -> f64 {
    (g_local * wavelength / (2.0 * std::f64::consts::PI)).sqrt()
}

pub fn shallow_water_phase_speed(g_local: f64, depth: f64) -> f64 {
    (g_local * depth).sqrt()
}

pub fn wave_dispersion(g_local: f64, k: f64, depth: f64) -> f64 {
    (g_local * k * (k * depth).tanh()).sqrt()
}

pub fn phillips_spectrum(
    k: f64,
    wind_speed: f64,
    g_local: f64,
    wind_direction_x: f64,
    wind_direction_y: f64,
    kx: f64,
    ky: f64,
) -> f64 {
    if k < 1e-12 {
        return 0.0;
    }
    let k_peak = g_local / wind_speed.powi(2);
    let l = wind_speed.powi(2) / g_local;
    let k2 = k * k;
    let cos_factor = (kx * wind_direction_x + ky * wind_direction_y) / k;
    if cos_factor <= 0.0 {
        return 0.0;
    }
    let phillips = PHILLIPS_ALPHA / (k2 * k2) * (-1.0 / (k2 * l * l)).exp();
    phillips * cos_factor.powi(2) * (-k / (k_peak * PHILLIPS_BETA.sqrt())).exp().max(0.0)
}

pub fn jonswap_spectrum(omega: f64, wind_speed: f64, fetch: f64, g_local: f64) -> f64 {
    let alpha =
        JONSWAP_ALPHA_COEFF * (wind_speed.powi(2) / (fetch * g_local)).powf(JONSWAP_ALPHA_EXPONENT);
    let omega_p = JONSWAP_FREQ_COEFF * (g_local.powi(2) / (wind_speed * fetch)).powf(1.0 / 3.0);
    let gamma = JONSWAP_GAMMA;
    let sigma: f64 = if omega <= omega_p {
        JONSWAP_SIGMA_A
    } else {
        JONSWAP_SIGMA_B
    };
    let r = (-(omega - omega_p).powi(2) / (2.0 * sigma.powi(2) * omega_p.powi(2))).exp();

    alpha * g_local.powi(2) / omega.powi(5)
        * (-1.25 * (omega_p / omega).powi(4)).exp()
        * gamma.powf(r)
}

pub fn significant_wave_height(wind_speed: f64, fetch: f64, g_local: f64) -> f64 {
    WAVE_HEIGHT_COEFF
        * (wind_speed.powi(2) / g_local)
        * (g_local * fetch / wind_speed.powi(2)).powf(WAVE_HEIGHT_EXPONENT)
}

pub fn wave_period(wind_speed: f64, fetch: f64, g_local: f64) -> f64 {
    WAVE_PERIOD_COEFF
        * (wind_speed / g_local)
        * (g_local * fetch / wind_speed.powi(2)).powf(1.0 / 3.0)
}

pub fn stokes_drift(amplitude: f64, omega: f64, k: f64, depth: f64, z: f64) -> f64 {
    amplitude.powi(2) * omega * k * (2.0 * k * (z + depth)).cosh()
        / (2.0 * (k * depth).sinh().powi(2))
}

pub fn ekman_transport(wind_stress: f64, coriolis_parameter: f64, density: f64) -> f64 {
    wind_stress / (density * coriolis_parameter)
}

pub fn ekman_spiral_velocity(surface_current: f64, ekman_depth: f64, z: f64) -> (f64, f64) {
    let a = std::f64::consts::PI * z / ekman_depth;
    let decay = (a).exp();
    let u = surface_current * decay * (std::f64::consts::FRAC_PI_4 - a).cos();
    let v = surface_current * decay * (std::f64::consts::FRAC_PI_4 - a).sin();
    (u, v)
}

pub fn thermohaline_density(
    rho_0: f64,
    alpha_t: f64,
    beta_s: f64,
    temperature: f64,
    t_ref: f64,
    salinity: f64,
    s_ref: f64,
) -> f64 {
    rho_0 * (1.0 - alpha_t * (temperature - t_ref) + beta_s * (salinity - s_ref))
}

pub fn overturning_stream_function(
    thermal_forcing: f64,
    salinity_forcing: f64,
    alpha_t: f64,
    beta_s: f64,
    kappa: f64,
    depth: f64,
) -> f64 {
    let buoyancy = alpha_t * thermal_forcing - beta_s * salinity_forcing;
    (buoyancy * kappa * depth.powi(3)).abs().powf(1.0 / 3.0) * buoyancy.signum()
}

pub fn mixed_layer_depth(wind_stress: f64, buoyancy_flux: f64, coriolis: f64) -> f64 {
    if buoyancy_flux <= 0.0 {
        return 0.0;
    }
    (2.0 * wind_stress / (coriolis * buoyancy_flux.abs())).sqrt()
}

pub fn tidal_amplitude(
    mass_perturber: f64,
    distance: f64,
    body_radius: f64,
    ocean_depth: f64,
    g_surface: f64,
) -> f64 {
    let potential = G * mass_perturber * body_radius.powi(2) / distance.powi(3);
    potential / g_surface * (ocean_depth / (g_surface * body_radius)).sqrt()
}

pub fn geostrophic_current(g_local: f64, sea_surface_slope: f64, coriolis: f64) -> f64 {
    -g_local * sea_surface_slope / coriolis
}

pub fn upwelling_velocity(wind_stress_curl: f64, coriolis: f64, density: f64) -> f64 {
    wind_stress_curl / (density * coriolis)
}

pub fn ocean_heat_content(
    density: f64,
    specific_heat: f64,
    delta_t: f64,
    depth: f64,
    area: f64,
) -> f64 {
    density * specific_heat * delta_t * depth * area
}

pub fn sea_ice_growth_rate(
    thermal_conductivity_ice: f64,
    ice_thickness: f64,
    surface_temp: f64,
    freezing_point: f64,
    latent_heat: f64,
    ice_density: f64,
) -> f64 {
    if surface_temp >= freezing_point {
        return 0.0;
    }
    thermal_conductivity_ice * (freezing_point - surface_temp)
        / (latent_heat * ice_density * ice_thickness.max(0.01))
}

pub fn internal_wave_speed(
    g_local: f64,
    density_upper: f64,
    density_lower: f64,
    depth_upper: f64,
    depth_lower: f64,
) -> f64 {
    let reduced_g =
        g_local * (density_lower - density_upper) / ((density_lower + density_upper) / 2.0);
    (reduced_g * depth_upper * depth_lower / (depth_upper + depth_lower)).sqrt()
}

pub fn langmuir_circulation_depth(wind_speed: f64, surface_current: f64) -> f64 {
    let la = (surface_current / wind_speed).sqrt();
    if la < 1e-6 {
        return 0.0;
    }
    wind_speed.powi(2) / (EARTH_GRAVITY * la)
}

pub fn seawater_density(t: f64, s: f64) -> f64 {
    let rho_w = SEAWATER_RHO_0
        + SEAWATER_RHO_T1 * t
        + SEAWATER_RHO_T2 * t.powi(2)
        + SEAWATER_RHO_T3 * t.powi(3)
        + SEAWATER_RHO_T4 * t.powi(4)
        + SEAWATER_RHO_T5 * t.powi(5);
    let a = SEAWATER_RHO_S0
        + SEAWATER_RHO_S_T1 * t
        + SEAWATER_RHO_S_T2 * t.powi(2)
        + SEAWATER_RHO_S_T3 * t.powi(3)
        + SEAWATER_RHO_S_T4 * t.powi(4);
    let b = SEAWATER_RHO_B0 + SEAWATER_RHO_B_T1 * t + SEAWATER_RHO_B_T2 * t.powi(2);
    let c = SEAWATER_RHO_C0;
    rho_w + a * s + b * s.powf(1.5) + c * s.powi(2)
}

pub fn sound_speed(t: f64, s: f64, d: f64) -> f64 {
    SOUND_SPEED_BASE
        + SOUND_SPEED_T1 * t
        + SOUND_SPEED_T2 * t.powi(2)
        + SOUND_SPEED_T3 * t.powi(3)
        + SOUND_SPEED_S1 * (s - 35.0)
        + SOUND_SPEED_D1 * d
        + SOUND_SPEED_D2 * d.powi(2)
        + SOUND_SPEED_TS * t * (s - 35.0)
        + SOUND_SPEED_TD * t * d.powi(3)
}

pub fn sea_level_rise_thermal(alpha: f64, delta_t: f64, d: f64) -> f64 {
    alpha * delta_t * d
}

pub fn henry_solubility(kh: f64, t: f64, delta_h: f64) -> f64 {
    kh * ((-delta_h / R_GAS) * (1.0 / t - 1.0 / 298.15)).exp()
}

pub fn haline_contraction_coefficient(salinity_psu: f64) -> f64 {
    HALINE_CONTRACTION_0 + HALINE_CONTRACTION_1 * salinity_psu
}

pub fn thermal_expansion_coefficient_sw(temperature_c: f64) -> f64 {
    (THERMAL_EXPANSION_0 + THERMAL_EXPANSION_1 * temperature_c).max(0.0)
}
