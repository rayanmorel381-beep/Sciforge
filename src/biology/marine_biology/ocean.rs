pub fn osmoregulation_flux(
    internal_osmolarity: f64,
    external_osmolarity: f64,
    permeability: f64,
    surface_area: f64,
) -> f64 {
    permeability * surface_area * (external_osmolarity - internal_osmolarity)
}

pub fn coral_bleaching_threshold(sst: f64, mmm: f64) -> f64 {
    (sst - mmm - 1.0).max(0.0)
}

pub fn degree_heating_weeks(weekly_anomalies: &[f64]) -> f64 {
    weekly_anomalies.iter().map(|&a| a.max(0.0)).sum()
}

pub fn ocean_acidification_ph(pco2: f64, alkalinity: f64, temperature: f64) -> f64 {
    let k = 10.0_f64.powf(-6.35 + 0.01 * (temperature - 25.0));
    let h = (k * pco2 / alkalinity).sqrt();
    -h.log10()
}

pub fn carbonate_saturation_state(ca_concentration: f64, co3_concentration: f64, ksp: f64) -> f64 {
    ca_concentration * co3_concentration / ksp
}

pub fn bioluminescence_intensity(luciferin: f64, luciferase: f64, oxygen: f64, km: f64) -> f64 {
    luciferase * luciferin * oxygen / (km + oxygen)
}

pub fn depth_light_attenuation(surface_irradiance: f64, attenuation_coeff: f64, depth: f64) -> f64 {
    surface_irradiance * (-attenuation_coeff * depth).exp()
}

pub fn thermohaline_density(temperature: f64, salinity: f64) -> f64 {
    999.842594 + 6.793952e-2 * temperature - 9.095290e-3 * temperature.powi(2)
        + 1.001685e-4 * temperature.powi(3)
        + 0.824493 * salinity
        - 4.0899e-3 * temperature * salinity
}

pub fn mixed_layer_depth_temperature(
    profile_temps: &[f64],
    profile_depths: &[f64],
    threshold: f64,
) -> f64 {
    if profile_temps.is_empty() {
        return 0.0;
    }
    let sst = profile_temps[0];
    for i in 1..profile_temps.len() {
        if (sst - profile_temps[i]).abs() > threshold {
            return profile_depths[i];
        }
    }
    *profile_depths.last().unwrap_or(&0.0)
}

pub fn ekman_depth(coriolis: f64, eddy_viscosity: f64) -> f64 {
    std::f64::consts::PI * (2.0 * eddy_viscosity / coriolis.abs().max(1e-30)).sqrt()
}

pub fn ekman_transport(wind_stress: f64, coriolis: f64, density: f64) -> f64 {
    wind_stress / (density * coriolis.abs().max(1e-30))
}

pub fn sverdrup_transport(wind_stress_curl: f64, beta: f64) -> f64 {
    wind_stress_curl / beta
}

pub fn primary_production_eppley(chlorophyll: f64, par: f64, temperature: f64) -> f64 {
    let p_max = 0.59 * (0.0633 * temperature).exp();
    p_max * chlorophyll * (1.0 - (-par / (p_max * 10.0).max(1e-30)).exp())
}

pub fn new_production_f_ratio(nitrate_uptake: f64, total_uptake: f64) -> f64 {
    nitrate_uptake / total_uptake.max(1e-30)
}

pub fn nitrogen_fixation_rate(temperature: f64, iron: f64, light: f64) -> f64 {
    let temp_factor = (-0.05 * (temperature - 27.0).powi(2)).exp();
    let iron_factor = iron / (iron + 0.2e-9);
    let light_factor = light / (light + 50.0);
    temp_factor * iron_factor * light_factor
}

pub fn oxygen_minimum_zone_depth(
    surface_o2: f64,
    respiration_rate: f64,
    ventilation_rate: f64,
) -> f64 {
    surface_o2 / (respiration_rate - ventilation_rate).max(1e-30)
}

pub fn seawater_sound_speed(temperature: f64, salinity: f64, depth: f64) -> f64 {
    1449.2 + 4.6 * temperature - 0.055 * temperature.powi(2)
        + 0.00029 * temperature.powi(3)
        + (1.34 - 0.01 * temperature) * (salinity - 35.0)
        + 0.016 * depth
}

pub fn coral_calcification_rate(aragonite_saturation: f64, temperature: f64, light: f64) -> f64 {
    let sat_factor = (aragonite_saturation - 1.0).max(0.0);
    let temp_factor = (-0.1 * (temperature - 26.0).powi(2)).exp();
    let light_factor = light / (light + 200.0);
    sat_factor * temp_factor * light_factor
}

pub fn tidal_range(lunar_amplitude: f64, solar_amplitude: f64, phase_angle: f64) -> f64 {
    2.0 * (lunar_amplitude.powi(2)
        + solar_amplitude.powi(2)
        + 2.0 * lunar_amplitude * solar_amplitude * phase_angle.cos())
    .sqrt()
}

pub fn wave_energy_density(density: f64, gravity: f64, wave_height: f64) -> f64 {
    density * gravity * wave_height.powi(2) / 16.0
}

pub fn deep_water_wave_speed(gravity: f64, wavelength: f64) -> f64 {
    (gravity * wavelength / (2.0 * std::f64::consts::PI)).sqrt()
}

pub fn upwelling_velocity(
    wind_stress: f64,
    coriolis: f64,
    density: f64,
    coast_distance: f64,
) -> f64 {
    wind_stress / (density * coriolis.abs().max(1e-30) * coast_distance)
}
