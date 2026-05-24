use crate::constants::{G, K_B, MU_0};

pub fn rayleigh_number(
    alpha: f64,
    g_surface: f64,
    delta_t: f64,
    depth: f64,
    kappa: f64,
    nu: f64,
) -> f64 {
    alpha * g_surface * delta_t * depth.powi(3) / (kappa * nu)
}

pub fn nusselt_number(rayleigh: f64, beta_exponent: f64) -> f64 {
    0.195 * rayleigh.powf(beta_exponent)
}

pub fn convective_velocity(
    alpha: f64,
    g_surface: f64,
    delta_t: f64,
    depth: f64,
    kappa: f64,
    nu: f64,
) -> f64 {
    let ra = rayleigh_number(alpha, g_surface, delta_t, depth, kappa, nu);
    (kappa / depth) * ra.powf(2.0 / 3.0)
}

pub fn core_temperature_adiabat(
    surface_core_temp: f64,
    pressure: f64,
    gruneisen_parameter: f64,
    bulk_modulus: f64,
) -> f64 {
    surface_core_temp * (1.0 + gruneisen_parameter * pressure / bulk_modulus)
}

pub fn inner_core_radius(
    total_core_radius: f64,
    core_mass: f64,
    melting_gradient: f64,
    adiabatic_gradient: f64,
) -> f64 {
    if adiabatic_gradient >= melting_gradient {
        return 0.0;
    }
    let pressure_factor = (G * core_mass / total_core_radius.powi(2)).cbrt();
    let fraction =
        (1.0 - adiabatic_gradient / melting_gradient) * pressure_factor / pressure_factor.max(1.0);
    total_core_radius * fraction.sqrt()
}

pub fn core_heat_flux(thermal_conductivity: f64, delta_t: f64, core_radius: f64) -> f64 {
    thermal_conductivity * delta_t / core_radius
}

pub fn cmb_heat_flux(
    nusselt: f64,
    thermal_conductivity: f64,
    delta_t: f64,
    mantle_depth: f64,
) -> f64 {
    nusselt * thermal_conductivity * delta_t / mantle_depth
}

pub fn mantle_overturn_time(mantle_depth: f64, convective_vel: f64) -> f64 {
    mantle_depth / convective_vel
}

pub fn viscosity_temperature(
    reference_viscosity: f64,
    activation_energy: f64,
    temperature: f64,
    reference_temperature: f64,
) -> f64 {
    reference_viscosity
        * ((activation_energy / (K_B * temperature))
            - (activation_energy / (K_B * reference_temperature)))
            .exp()
}

pub fn thermal_boundary_layer_thickness(mantle_depth: f64, rayleigh: f64) -> f64 {
    mantle_depth * rayleigh.powf(-1.0 / 3.0)
}

pub fn plume_buoyancy_flux(
    alpha: f64,
    density: f64,
    g_surface: f64,
    delta_t: f64,
    volume_flux: f64,
) -> f64 {
    alpha * density * g_surface * delta_t * volume_flux
}

pub fn plume_conduit_radius(
    buoyancy_flux: f64,
    kinematic_viscosity: f64,
    thermal_diffusivity: f64,
) -> f64 {
    (buoyancy_flux * kinematic_viscosity / thermal_diffusivity).powf(0.25)
}

pub fn core_cooling_rate(
    surface_heat_flux: f64,
    core_radius: f64,
    core_density: f64,
    specific_heat: f64,
) -> f64 {
    3.0 * surface_heat_flux / (core_radius * core_density * specific_heat)
}

pub fn magnetic_reynolds_number(
    velocity: f64,
    length_scale: f64,
    magnetic_diffusivity: f64,
) -> f64 {
    velocity * length_scale / magnetic_diffusivity
}

pub fn dynamo_active(magnetic_reynolds: f64) -> bool {
    magnetic_reynolds > 40.0
}

pub fn dipole_moment(
    core_radius: f64,
    density: f64,
    angular_velocity: f64,
    conductivity: f64,
    convective_power: f64,
) -> f64 {
    let prefactor = 4.0 * std::f64::consts::PI * core_radius.powi(3);
    let buoyancy_factor =
        (density * angular_velocity * convective_power / (MU_0 * conductivity)).sqrt();
    prefactor * buoyancy_factor
}

pub fn convective_power(cmb_flux: f64, core_radius: f64) -> f64 {
    4.0 * std::f64::consts::PI * core_radius.powi(2) * cmb_flux
}

pub fn tidal_heating_rate(
    mass: f64,
    radius: f64,
    eccentricity: f64,
    orbital_freq: f64,
    tidal_q: f64,
    rigidity: f64,
) -> f64 {
    let k2 = 1.5 / (1.0 + 19.0 * rigidity / (2.0 * G * mass * mass / (radius.powi(4))));
    21.0 / 2.0
        * k2
        * G
        * mass.powi(2)
        * radius.powi(5)
        * orbital_freq.powi(5)
        * eccentricity.powi(2)
        / tidal_q
}

pub fn radiogenic_heating(
    mass_mantle: f64,
    concentration_u238: f64,
    concentration_th232: f64,
    concentration_k40: f64,
    time_ga: f64,
) -> f64 {
    let lambda_u238 = crate::constants::LAMBDA_U238;
    let lambda_th232 = crate::constants::LAMBDA_TH232;
    let lambda_k40 = crate::constants::LAMBDA_K40_TOTAL;
    let h_u238 = crate::constants::HEAT_PRODUCTION_U238;
    let h_th232 = crate::constants::HEAT_PRODUCTION_TH232;
    let h_k40 = crate::constants::HEAT_PRODUCTION_K40;

    let t_s = time_ga * 1e9 * 365.25 * 86400.0;

    mass_mantle
        * (concentration_u238 * h_u238 * (lambda_u238 * t_s).exp()
            + concentration_th232 * h_th232 * (lambda_th232 * t_s).exp()
            + concentration_k40 * h_k40 * (lambda_k40 * t_s).exp())
}

pub fn secular_cooling_flux(
    core_heat_capacity: f64,
    core_mass: f64,
    cooling_rate_k_per_s: f64,
    core_radius: f64,
) -> f64 {
    core_heat_capacity * core_mass * cooling_rate_k_per_s
        / (4.0 * std::f64::consts::PI * core_radius.powi(2))
}

pub fn gravitational_differentiation_power(
    inner_core_growth_rate: f64,
    density_contrast: f64,
    g_cmb: f64,
    core_radius: f64,
) -> f64 {
    inner_core_growth_rate * density_contrast * g_cmb * core_radius
}

pub fn parameterized_mantle_temperature(
    initial_temp: f64,
    radiogenic_heat: f64,
    surface_heat_loss: f64,
    mantle_heat_capacity: f64,
    mantle_mass: f64,
    dt: f64,
) -> f64 {
    initial_temp + (radiogenic_heat - surface_heat_loss) * dt / (mantle_heat_capacity * mantle_mass)
}

pub fn stagnant_lid_thickness(
    mantle_depth: f64,
    activation_energy: f64,
    delta_t: f64,
    interior_temp: f64,
) -> f64 {
    let theta = activation_energy * delta_t / (K_B * interior_temp.powi(2));
    mantle_depth / theta.max(1.0)
}

pub fn planet_surface_heat_flux(
    interior_temp: f64,
    surface_temp: f64,
    thermal_conductivity: f64,
    lithosphere_thickness: f64,
) -> f64 {
    thermal_conductivity * (interior_temp - surface_temp) / lithosphere_thickness
}

pub fn core_liquidus(pressure_gpa: f64, fe_fraction: f64) -> f64 {
    1811.0 + 30.0 * pressure_gpa - 1000.0 * (1.0 - fe_fraction)
}
