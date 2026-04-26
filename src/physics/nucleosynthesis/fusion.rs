use crate::constants::{C, EPSILON_0, G, K_B, MU_0, SIGMA_SB};

pub fn pp_chain_rate(temperature_k: f64, density_kg_m3: f64, hydrogen_fraction: f64) -> f64 {
    let t9 = temperature_k / 1e9;
    if t9 <= 0.0 {
        return 0.0;
    }
    let screening = 1.0 + 0.0123 * (density_kg_m3 / 1e5).powf(1.0 / 3.0) / t9;
    let gamow = (-3.380 / t9.powf(1.0 / 3.0)).exp();
    let s11 = 4.01e-22;
    let rate = s11 * gamow * screening / t9.powf(2.0 / 3.0);
    let energy_per_chain = 26.198 * 1.602_176_634e-13;
    rate * density_kg_m3 * hydrogen_fraction.powi(2) * energy_per_chain
}

pub fn cno_cycle_rate(
    temperature_k: f64,
    density_kg_m3: f64,
    hydrogen_fraction: f64,
    cno_fraction: f64,
) -> f64 {
    let t9 = temperature_k / 1e9;
    if t9 <= 0.0 {
        return 0.0;
    }
    let gamow = (-15.228 / t9.powf(1.0 / 3.0)).exp();
    let rate = 8.67e-17 * gamow / t9.powf(2.0 / 3.0);
    let energy_per_cycle = 25.03 * 1.602_176_634e-13;
    rate * density_kg_m3 * hydrogen_fraction * cno_fraction * energy_per_cycle
}

pub fn triple_alpha_rate(temperature_k: f64, density_kg_m3: f64, helium_fraction: f64) -> f64 {
    let t9 = temperature_k / 1e9;
    if t9 <= 0.0 {
        return 0.0;
    }
    let rate = 7.831e-15 * (-4.4027 / t9).exp() / t9.powi(3);
    let energy = 7.275 * 1.602_176_634e-13;
    rate * density_kg_m3.powi(2) * helium_fraction.powi(3) * energy
}

pub fn nuclear_energy_generation(
    temperature_k: f64,
    density_kg_m3: f64,
    hydrogen_fraction: f64,
    helium_fraction: f64,
    metal_fraction: f64,
) -> f64 {
    let pp = pp_chain_rate(temperature_k, density_kg_m3, hydrogen_fraction);
    let cno = cno_cycle_rate(
        temperature_k,
        density_kg_m3,
        hydrogen_fraction,
        metal_fraction,
    );
    let ta = triple_alpha_rate(temperature_k, density_kg_m3, helium_fraction);
    pp + cno + ta
}

pub fn thermal_pressure(electron_density: f64, temperature_k: f64) -> f64 {
    2.0 * electron_density * K_B * temperature_k
}

pub fn magnetic_pressure(magnetic_field: f64) -> f64 {
    magnetic_field.powi(2) / (2.0 * MU_0)
}

pub fn plasma_beta(electron_density: f64, temperature_k: f64, magnetic_field: f64) -> f64 {
    thermal_pressure(electron_density, temperature_k) / magnetic_pressure(magnetic_field)
}

pub fn alfven_speed(magnetic_field: f64, density: f64) -> f64 {
    magnetic_field / (MU_0 * density).sqrt()
}

pub fn plasma_frequency(electron_density: f64) -> f64 {
    let e = 1.602_176_634e-19;
    let m_e = 9.109_383_7e-31;
    (electron_density * e * e / (EPSILON_0 * m_e)).sqrt()
}

pub fn debye_length(electron_density: f64, temperature_k: f64) -> f64 {
    let e = 1.602_176_634e-19;
    (EPSILON_0 * K_B * temperature_k / (electron_density * e * e)).sqrt()
}

pub fn radiative_loss_rate(electron_density: f64, temperature_k: f64) -> f64 {
    let lambda = if temperature_k < 1e5 {
        1e-26
    } else if temperature_k < 1e7 {
        1e-22 * (temperature_k / 1e6).powf(-0.7)
    } else {
        1e-23 * (temperature_k / 1e7).powf(0.5)
    };
    electron_density.powi(2) * lambda
}

pub fn thermal_conduction_flux(temperature_k: f64, length_scale: f64) -> f64 {
    let kappa_0 = 9.2e-12;
    kappa_0 * temperature_k.powf(3.5) / length_scale
}

pub fn sound_speed_plasma(temperature_k: f64, mean_particle_mass: f64) -> f64 {
    let gamma = 5.0 / 3.0;
    (gamma * K_B * temperature_k / mean_particle_mass).sqrt()
}

pub fn gyrofrequency(charge: f64, magnetic_field: f64, mass: f64) -> f64 {
    charge.abs() * magnetic_field / mass
}

pub fn larmor_radius(mass: f64, velocity_perp: f64, charge: f64, magnetic_field: f64) -> f64 {
    mass * velocity_perp / (charge.abs() * magnetic_field)
}

pub fn reconnection_rate_sweet_parker(alfven_speed_val: f64, lundquist_number: f64) -> f64 {
    alfven_speed_val / lundquist_number.sqrt()
}

pub fn lundquist_number(
    alfven_speed_val: f64,
    length_scale: f64,
    magnetic_diffusivity: f64,
) -> f64 {
    alfven_speed_val * length_scale / magnetic_diffusivity
}

pub fn coronal_loop_temperature(loop_length: f64, heating_rate: f64) -> f64 {
    1400.0 * (heating_rate * loop_length.powi(2)).powf(1.0 / 3.0)
}

pub fn coronal_loop_density(heating_rate: f64, loop_length: f64, temperature_k: f64) -> f64 {
    let kappa_0 = 9.2e-12;
    heating_rate * loop_length / (2.0 * kappa_0 * temperature_k.powf(3.5) / loop_length) * 1e6
}

pub fn solar_flare_energy(magnetic_field: f64, volume: f64) -> f64 {
    magnetic_pressure(magnetic_field) * volume
}

pub fn cme_kinetic_energy(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * velocity.powi(2)
}

pub fn sunspot_temperature(photosphere_temp: f64, suppression_factor: f64) -> f64 {
    photosphere_temp * (1.0 - suppression_factor).powf(0.25)
}

pub fn differential_rotation_rate(equatorial_rate: f64, latitude: f64, a2: f64, a4: f64) -> f64 {
    equatorial_rate - a2 * latitude.sin().powi(2) - a4 * latitude.sin().powi(4)
}

pub fn stellar_wind_mass_loss_si(luminosity: f64, escape_velocity: f64) -> f64 {
    1e-14 * luminosity / (C * escape_velocity)
}

pub fn convective_envelope_depth(stellar_mass_solar: f64) -> f64 {
    if stellar_mass_solar < 0.35 {
        1.0
    } else if stellar_mass_solar < 1.2 {
        0.3 * (1.2 - stellar_mass_solar) / (1.2 - 0.35)
    } else {
        0.0
    }
}

pub fn mixing_length_velocity(
    convective_flux: f64,
    density: f64,
    temperature: f64,
    pressure: f64,
    mixing_length: f64,
    g_local: f64,
    cp: f64,
) -> f64 {
    let nabla_excess =
        convective_flux / (density * cp * temperature * (pressure / (density * g_local)));
    (g_local * mixing_length.powi(2) * nabla_excess / temperature).powf(1.0 / 3.0)
}

pub fn opacity_kramers(
    density: f64,
    temperature: f64,
    hydrogen_fraction: f64,
    metal_fraction: f64,
) -> f64 {
    let kappa_ff = 3.68e22
        * (hydrogen_fraction + 0.2 * (1.0 + hydrogen_fraction))
        * density
        * temperature.powf(-3.5);
    let kappa_bf =
        4.34e25 * metal_fraction * (1.0 + hydrogen_fraction) * density * temperature.powf(-3.5);
    kappa_ff + kappa_bf
}

pub fn radiative_temperature_gradient(
    opacity: f64,
    luminosity: f64,
    mass_enclosed: f64,
    pressure: f64,
    temperature: f64,
) -> f64 {
    3.0 * opacity * luminosity * pressure
        / (16.0 * std::f64::consts::PI * SIGMA_SB * C * G * mass_enclosed * temperature.powi(4))
}
