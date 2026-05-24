use crate::constants::{G, R_GAS};

pub fn magma_viscosity(a: f64, activation_energy: f64, temperature_k: f64) -> f64 {
    a * (activation_energy / (R_GAS * temperature_k)).exp()
}

pub fn magma_buoyancy_force(
    rho_magma: f64,
    rho_crust: f64,
    g_surface: f64,
    chamber_volume: f64,
) -> f64 {
    (rho_crust - rho_magma) * g_surface * chamber_volume
}

pub fn overpressure_threshold(tensile_strength: f64, lithostatic_pressure: f64) -> f64 {
    lithostatic_pressure + tensile_strength
}

pub fn chamber_overpressure(bulk_modulus: f64, volume_injected: f64, chamber_volume: f64) -> f64 {
    bulk_modulus * volume_injected / chamber_volume
}

pub fn effusion_rate(
    overpressure: f64,
    conduit_radius: f64,
    conduit_length: f64,
    viscosity: f64,
) -> f64 {
    std::f64::consts::PI * conduit_radius.powi(4) * overpressure
        / (8.0 * viscosity * conduit_length)
}

pub fn volcanic_explosivity_index(ejecta_volume_km3: f64) -> f64 {
    if ejecta_volume_km3 <= 0.0 {
        return 0.0;
    }
    (ejecta_volume_km3.log10() + 4.0).clamp(0.0, 8.0)
}

pub fn lava_flow_velocity(
    g_surface: f64,
    slope_angle: f64,
    thickness: f64,
    density: f64,
    viscosity: f64,
) -> f64 {
    density * g_surface * slope_angle.sin() * thickness.powi(2) / (3.0 * viscosity)
}

pub fn pyroclastic_column_height(thermal_flux: f64) -> f64 {
    0.258 * thermal_flux.powf(0.25)
}

pub fn seismic_to_eruption_probability(
    cumulative_seismic_moment: f64,
    critical_moment: f64,
    b_value: f64,
) -> f64 {
    let ratio = cumulative_seismic_moment / critical_moment;
    (1.0 - (-b_value * ratio).exp()).clamp(0.0, 1.0)
}

pub fn tidal_triggering_stress(mass_perturber: f64, distance: f64, body_radius: f64) -> f64 {
    2.0 * G * mass_perturber * body_radius / distance.powi(3)
}

pub fn coulomb_failure_stress(
    shear_stress: f64,
    normal_stress: f64,
    pore_pressure: f64,
    friction_coefficient: f64,
    cohesion: f64,
) -> f64 {
    shear_stress - friction_coefficient * (normal_stress - pore_pressure) - cohesion
}

pub fn magma_ascent_velocity(
    density_contrast: f64,
    g_surface: f64,
    conduit_radius: f64,
    viscosity: f64,
) -> f64 {
    density_contrast * g_surface * conduit_radius.powi(2) / (8.0 * viscosity)
}

pub fn volatile_exsolution_depth(saturation_pressure: f64, density: f64, g_surface: f64) -> f64 {
    saturation_pressure / (density * g_surface)
}

pub fn fragmentation_threshold(porosity: f64, vesicularity_critical: f64) -> bool {
    porosity >= vesicularity_critical
}

pub fn tephra_fallout_distance(
    column_height: f64,
    wind_speed: f64,
    particle_settling_velocity: f64,
) -> f64 {
    column_height * wind_speed / particle_settling_velocity
}

pub fn thermal_erosion_rate(
    lava_temperature: f64,
    substrate_solidus: f64,
    lava_velocity: f64,
    thermal_diffusivity: f64,
) -> f64 {
    let superheat = (lava_temperature - substrate_solidus).max(0.0);
    0.001 * superheat * lava_velocity * thermal_diffusivity.sqrt()
}

pub fn degassing_rate(
    volatile_concentration: f64,
    solubility_coefficient: f64,
    pressure: f64,
) -> f64 {
    let saturation = solubility_coefficient * pressure.sqrt();
    (volatile_concentration - saturation).max(0.0)
}

pub fn eruption_energy(ejecta_mass: f64, ejecta_velocity: f64, thermal_energy: f64) -> f64 {
    0.5 * ejecta_mass * ejecta_velocity.powi(2) + thermal_energy
}
