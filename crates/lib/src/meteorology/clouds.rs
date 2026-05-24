use crate::constants::{MAGNUS_A, MAGNUS_B, STANDARD_AIR_DENSITY_SEA_LEVEL};

pub fn saturation_vapor_pressure(temperature_k: f64) -> f64 {
    let t_c = temperature_k - 273.15;
    611.2 * (MAGNUS_A * t_c / (t_c + MAGNUS_B)).exp()
}

pub fn cloud_base_altitude(surface_temperature: f64, dew_point: f64, lapse_rate: f64) -> f64 {
    (surface_temperature - dew_point) / lapse_rate
}

pub fn adiabatic_liquid_water_content(
    altitude_above_base: f64,
    lapse_rate: f64,
    temperature_at_base: f64,
) -> f64 {
    let rho_air = STANDARD_AIR_DENSITY_SEA_LEVEL;
    let dqs_dt = 0.0004;
    rho_air * dqs_dt * lapse_rate * altitude_above_base / temperature_at_base.max(1.0)
}

pub fn cloud_optical_depth(liquid_water_path: f64, effective_radius: f64) -> f64 {
    3.0 * liquid_water_path / (2.0 * 1000.0 * effective_radius)
}

pub fn cloud_albedo(optical_depth: f64) -> f64 {
    let g = 0.85;
    let tau_star = optical_depth * (1.0 - g);
    tau_star / (tau_star + 2.0)
}

pub fn droplet_growth_condensation(
    supersaturation: f64,
    radius: f64,
    temperature: f64,
    pressure: f64,
) -> f64 {
    let rv = crate::constants::SPECIFIC_GAS_CONSTANT_WATER_VAPOR;
    let l_v = 2.5e6;
    let ka = 0.024;
    let dv_ref = 2.21e-5;
    let p_ref = crate::constants::P_STD;
    let dv = dv_ref * (p_ref / pressure) * (temperature / 273.15).powf(1.94);
    let es = saturation_vapor_pressure(temperature);

    let fk = (l_v / (rv * temperature) - 1.0) * l_v / (ka * temperature);
    let fd = rv * temperature / (dv * es);

    supersaturation / (radius * (fk + fd))
}

pub fn droplet_growth_collision(
    collector_radius: f64,
    collected_radius: f64,
    liquid_water_content: f64,
    collection_efficiency: f64,
    density_water: f64,
    terminal_velocity_diff: f64,
) -> f64 {
    let pi = std::f64::consts::PI;
    collection_efficiency
        * pi
        * (collector_radius + collected_radius).powi(2)
        * terminal_velocity_diff
        * liquid_water_content
        / (4.0 * density_water)
}

pub fn autoconversion_rate(lwc: f64, droplet_concentration: f64, threshold_lwc: f64) -> f64 {
    if lwc <= threshold_lwc {
        return 0.0;
    }
    1.67e-5 * (lwc - threshold_lwc) / droplet_concentration.powf(1.0 / 3.0)
}

pub fn ice_crystal_growth_rate(temperature_k: f64, supersaturation_ice: f64) -> f64 {
    let t_c = temperature_k - 273.15;
    let habit_factor = if t_c > -4.0 {
        1.0
    } else if t_c > -10.0 {
        2.0
    } else if t_c > -22.0 {
        1.5
    } else {
        1.0
    };
    1e-7 * supersaturation_ice * habit_factor * (-4000.0 / temperature_k).exp()
}

pub fn bergeron_process_rate(ice_crystal_density: f64, lwc: f64, temperature_k: f64) -> f64 {
    let es_water = saturation_vapor_pressure(temperature_k);
    let es_ice = es_water * (-6132.9 / temperature_k + 21.87).exp()
        / (-5417.98 / temperature_k + 19.83).exp();
    let delta_e = (es_water - es_ice).max(0.0);
    ice_crystal_density * delta_e * lwc * 1e-10
}

pub fn cloud_layer_temperature(
    base_temperature: f64,
    lapse_rate: f64,
    altitude_above_base: f64,
) -> f64 {
    base_temperature - lapse_rate * altitude_above_base
}

pub fn mie_size_parameter(particle_radius: f64, wavelength: f64) -> f64 {
    2.0 * std::f64::consts::PI * particle_radius / wavelength
}

pub fn extinction_efficiency(size_parameter: f64) -> f64 {
    if size_parameter < 0.01 {
        return 0.0;
    }
    if size_parameter < 1.0 {
        let x4 = size_parameter.powi(4);
        return 8.0 * x4 / 3.0;
    }
    2.0 + 4.0 / size_parameter * (2.0 * size_parameter).sin()
        + 4.0 / size_parameter.powi(2) * (1.0 - (2.0 * size_parameter).cos())
}

pub fn cloud_emissivity(optical_depth: f64) -> f64 {
    1.0 - (-optical_depth).exp()
}

pub fn convective_available_potential_energy(
    parcel_temperatures: &[f64],
    env_temperatures: &[f64],
    altitudes: &[f64],
    g_local: f64,
) -> f64 {
    let mut cape = 0.0;
    for i in 0..parcel_temperatures
        .len()
        .min(env_temperatures.len())
        .min(altitudes.len())
        - 1
    {
        let buoyancy =
            g_local * (parcel_temperatures[i] - env_temperatures[i]) / env_temperatures[i];
        if buoyancy > 0.0 {
            let dz = altitudes[i + 1] - altitudes[i];
            cape += buoyancy * dz;
        }
    }
    cape
}

pub fn convective_inhibition(
    parcel_temperatures: &[f64],
    env_temperatures: &[f64],
    altitudes: &[f64],
    g_local: f64,
) -> f64 {
    let mut cin = 0.0;
    for i in 0..parcel_temperatures
        .len()
        .min(env_temperatures.len())
        .min(altitudes.len())
        - 1
    {
        let buoyancy =
            g_local * (parcel_temperatures[i] - env_temperatures[i]) / env_temperatures[i];
        if buoyancy < 0.0 {
            let dz = altitudes[i + 1] - altitudes[i];
            cin += buoyancy * dz;
        }
    }
    cin.abs()
}

pub fn henyey_greenstein_phase(cos_theta: f64, asymmetry: f64) -> f64 {
    let g2 = asymmetry * asymmetry;
    (1.0 - g2) / (4.0 * std::f64::consts::PI * (1.0 + g2 - 2.0 * asymmetry * cos_theta).powf(1.5))
}

pub fn cloud_radiative_forcing(
    cloud_albedo: f64,
    surface_albedo: f64,
    solar_flux: f64,
    cloud_fraction: f64,
    longwave_trapping: f64,
) -> f64 {
    let sw_forcing = -cloud_fraction * solar_flux * (cloud_albedo - surface_albedo);
    let lw_forcing = cloud_fraction * longwave_trapping;
    sw_forcing + lw_forcing
}

pub fn nucleation_rate_ccn(supersaturation: f64, ccn_concentration: f64, k_exponent: f64) -> f64 {
    ccn_concentration * supersaturation.powf(k_exponent)
}

pub fn rain_evaporation_rate(rain_rate: f64, relative_humidity: f64, temperature: f64) -> f64 {
    if relative_humidity >= 1.0 {
        return 0.0;
    }
    let es = saturation_vapor_pressure(temperature);
    let ventilation = 1.6 + 30.3922 * rain_rate.powf(0.2046);
    1e-5 * ventilation * (1.0 - relative_humidity) * (es / 611.2) * rain_rate.powf(0.525)
}

pub fn terminal_velocity_droplet(radius_m: f64) -> f64 {
    if radius_m < 40e-6 {
        let k1 = 1.19e8;
        return k1 * radius_m.powi(2);
    }
    if radius_m < 600e-6 {
        let k2 = 8e3;
        return k2 * radius_m;
    }
    let k3 = 2.01e3;
    k3 * radius_m.powf(0.5)
}
