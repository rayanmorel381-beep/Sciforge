use crate::constants::{BEAUFORT_TO_MS_COEFF, BEAUFORT_TO_MS_EXPONENT, EARTH_GRAVITY, VON_KARMAN};

pub fn hadley_cell_extent(planet_radius: f64, rotation_rate: f64, delta_t: f64) -> f64 {
    ((5.0 * EARTH_GRAVITY * delta_t * planet_radius)
        / (3.0 * rotation_rate.powi(2) * planet_radius.powi(2)))
    .powf(0.25)
    .min(std::f64::consts::FRAC_PI_2)
}

pub fn thermal_wind_shear(g_local: f64, coriolis: f64, temperature: f64, dt_dy: f64) -> f64 {
    -g_local * dt_dy / (coriolis * temperature)
}

pub fn jet_stream_velocity(
    g_local: f64,
    delta_t: f64,
    meridional_distance: f64,
    coriolis: f64,
    scale_height: f64,
) -> f64 {
    g_local * delta_t * scale_height / (coriolis * temperature_mean(delta_t) * meridional_distance)
}

fn temperature_mean(delta_t: f64) -> f64 {
    250.0 + delta_t / 2.0
}

pub fn surface_wind_speed(pressure_gradient: f64, density: f64, drag_coefficient: f64) -> f64 {
    (pressure_gradient / (density * drag_coefficient)).sqrt()
}

pub fn wind_stress(air_density: f64, drag_coefficient: f64, wind_speed: f64) -> f64 {
    air_density * drag_coefficient * wind_speed.powi(2)
}

pub fn planetary_vorticity(rotation_rate: f64, latitude: f64) -> f64 {
    2.0 * rotation_rate * latitude.sin()
}

pub fn rossby_wave_phase_speed(beta: f64, zonal_wavenumber: f64, deformation_radius: f64) -> f64 {
    -beta / (zonal_wavenumber.powi(2) + 1.0 / deformation_radius.powi(2))
}

pub fn beta_parameter(rotation_rate: f64, planet_radius: f64, latitude: f64) -> f64 {
    2.0 * rotation_rate * latitude.cos() / planet_radius
}

pub fn baroclinic_instability_wavelength(deformation_radius: f64) -> f64 {
    2.0 * std::f64::consts::PI * 2.0_f64.sqrt() * deformation_radius
}

pub fn sea_breeze_speed(
    g_local: f64,
    delta_t: f64,
    boundary_layer_height: f64,
    mean_temperature: f64,
) -> f64 {
    (g_local * delta_t * boundary_layer_height / mean_temperature).sqrt()
}

pub fn katabatic_wind_speed(
    g_local: f64,
    delta_t: f64,
    mean_temperature: f64,
    slope_angle: f64,
    slope_length: f64,
    drag_coefficient: f64,
) -> f64 {
    let buoyancy = g_local * delta_t * slope_angle.sin() / mean_temperature;
    (buoyancy * slope_length / drag_coefficient).sqrt()
}

pub fn mountain_wave_vertical_velocity(
    wind_speed: f64,
    mountain_height: f64,
    brunt_vaisala: f64,
    horizontal_wavelength: f64,
) -> f64 {
    let k = 2.0 * std::f64::consts::PI / horizontal_wavelength;
    wind_speed * mountain_height * brunt_vaisala * k
}

pub fn ekman_pumping_velocity(wind_stress_curl: f64, density: f64, coriolis: f64) -> f64 {
    wind_stress_curl / (density * coriolis)
}

pub fn monin_obukhov_length(
    friction_velocity: f64,
    surface_temp: f64,
    sensible_heat_flux: f64,
    air_density: f64,
    specific_heat: f64,
) -> f64 {
    let k_von_karman = VON_KARMAN;
    -air_density * specific_heat * surface_temp * friction_velocity.powi(3)
        / (k_von_karman * EARTH_GRAVITY * sensible_heat_flux)
}

pub fn log_wind_profile(friction_velocity: f64, z: f64, roughness_length: f64) -> f64 {
    (friction_velocity / VON_KARMAN) * (z / roughness_length).ln()
}

pub fn cyclostrophic_wind(pressure_gradient: f64, density: f64, radius: f64) -> f64 {
    (pressure_gradient * radius / density).abs().sqrt()
}

pub fn gradient_wind(coriolis: f64, pressure_gradient: f64, density: f64, radius: f64) -> f64 {
    let discriminant = coriolis.powi(2) / 4.0 + pressure_gradient / (density * radius);
    if discriminant < 0.0 {
        return 0.0;
    }
    -coriolis / 2.0 + discriminant.sqrt()
}

pub fn superrotation_index(
    zonal_wind: f64,
    planet_radius: f64,
    rotation_rate: f64,
    latitude: f64,
) -> f64 {
    zonal_wind / (planet_radius * rotation_rate * latitude.cos()) - 1.0
}

pub fn foehn_warming(
    lapse_dry: f64,
    lapse_moist: f64,
    mountain_height: f64,
    condensation_level: f64,
) -> f64 {
    let ascent_dry = condensation_level * lapse_dry;
    let ascent_moist = (mountain_height - condensation_level) * lapse_moist;
    let descent_dry = mountain_height * lapse_dry;
    descent_dry - ascent_dry - ascent_moist
}

pub fn beaufort_to_m_s(b: f64) -> f64 {
    BEAUFORT_TO_MS_COEFF * b.powf(BEAUFORT_TO_MS_EXPONENT)
}

pub fn wind_chill(t: f64, v: f64) -> f64 {
    let v_kmh = v * 3.6;
    13.12 + 0.6215 * t - 11.37 * v_kmh.powf(0.16) + 0.3965 * t * v_kmh.powf(0.16)
}
