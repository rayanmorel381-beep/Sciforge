use crate::constants::R_GAS;

pub fn hypsometric_curve(mean_elevation: f64, std_dev: f64, fraction: f64) -> f64 {
    mean_elevation + std_dev * (2.0 * fraction - 1.0).asin() * 2.0 / std::f64::consts::PI
}

pub fn continental_fraction(total_area: f64, ocean_area: f64) -> f64 {
    1.0 - ocean_area / total_area
}

pub fn ocean_basin_depth(ridge_depth: f64, plate_age_myr: f64, thermal_diffusivity: f64) -> f64 {
    let t_s = plate_age_myr * 1e6 * 365.25 * 86400.0;
    ridge_depth
        + 2.0 * 1200.0 * (3300.0 - 1000.0) / 3300.0
            * (thermal_diffusivity * t_s / std::f64::consts::PI).sqrt()
}

pub fn mid_ocean_ridge_elevation(
    spreading_rate_cm_yr: f64,
    mantle_temperature: f64,
    reference_temperature: f64,
) -> f64 {
    let thermal_factor = (mantle_temperature - reference_temperature) / reference_temperature;
    2500.0 * (spreading_rate_cm_yr / 5.0).sqrt() * (1.0 + thermal_factor)
}

pub fn passive_margin_subsidence(
    initial_elevation: f64,
    time_myr: f64,
    thermal_time_constant_myr: f64,
) -> f64 {
    initial_elevation * (1.0 - (-time_myr / thermal_time_constant_myr).exp())
}

pub fn orogenic_elevation(
    convergence_rate: f64,
    erosion_rate: f64,
    crustal_density: f64,
    mantle_density: f64,
    time: f64,
) -> f64 {
    let thickening_rate = convergence_rate - erosion_rate;
    let isostatic_factor = 1.0 - crustal_density / mantle_density;
    thickening_rate * isostatic_factor * time
}

pub fn erosion_rate_hack(coefficient: f64, drainage_area: f64, slope: f64, m: f64, n: f64) -> f64 {
    coefficient * drainage_area.powf(m) * slope.powf(n)
}

pub fn sediment_transport_capacity(
    flow_velocity: f64,
    depth: f64,
    grain_size: f64,
    density_water: f64,
    density_sediment: f64,
    g_surface: f64,
) -> f64 {
    let shear_stress = density_water * g_surface * depth * (flow_velocity / depth).powi(2)
        / (flow_velocity.powi(2) + 1e-30);
    let shields = shear_stress / ((density_sediment - density_water) * g_surface * grain_size);
    let critical_shields = 0.047;
    if shields <= critical_shields {
        return 0.0;
    }
    8.0 * ((shields - critical_shields).powf(1.5))
        * ((density_sediment - density_water) * g_surface * grain_size.powi(3) / density_water)
            .sqrt()
}

pub fn crater_diameter(
    impactor_diameter: f64,
    impactor_velocity: f64,
    impactor_density: f64,
    target_density: f64,
    g_surface: f64,
) -> f64 {
    let energy = (4.0 / 3.0)
        * std::f64::consts::PI
        * (impactor_diameter / 2.0).powi(3)
        * impactor_density
        * impactor_velocity.powi(2)
        / 2.0;
    let energy_scale = (energy / 4.184e12).powf(0.294);
    1.161
        * (impactor_density / target_density).powf(1.0 / 3.0)
        * energy_scale
        * g_surface.powf(-0.22)
}

pub fn crater_depth_to_diameter(simple_to_complex_transition: f64, diameter: f64) -> f64 {
    if diameter < simple_to_complex_transition {
        diameter / 5.0
    } else {
        0.2 * simple_to_complex_transition.powf(0.3) * diameter.powf(0.7) / 5.0
    }
}

pub fn shield_volcano_profile(base_radius: f64, max_height: f64, r: f64) -> f64 {
    if r >= base_radius {
        return 0.0;
    }
    max_height * (1.0 - (r / base_radius).powi(2))
}

pub fn stratovolcano_profile(base_radius: f64, max_height: f64, r: f64) -> f64 {
    if r >= base_radius {
        return 0.0;
    }
    max_height * (1.0 - r / base_radius).powi(2)
}

pub fn caldera_profile(
    caldera_radius: f64,
    caldera_depth: f64,
    rim_height: f64,
    rim_width: f64,
    r: f64,
) -> f64 {
    if r < caldera_radius {
        -caldera_depth
    } else if r < caldera_radius + rim_width {
        let t = (r - caldera_radius) / rim_width;
        -caldera_depth + (caldera_depth + rim_height) * (std::f64::consts::PI * t / 2.0).sin()
    } else {
        rim_height * (-(r - caldera_radius - rim_width).powi(2) / (2.0 * rim_width.powi(2))).exp()
    }
}

pub fn rift_valley_width(
    elastic_thickness: f64,
    youngs_modulus: f64,
    poisson: f64,
    mantle_density: f64,
    g_surface: f64,
) -> f64 {
    let d = youngs_modulus * elastic_thickness.powi(3) / (12.0 * (1.0 - poisson.powi(2)));
    std::f64::consts::PI * (4.0 * d / (mantle_density * g_surface)).powf(0.25)
}

pub fn tectonic_stress_field(plate_velocity: f64, viscosity: f64, thickness: f64) -> f64 {
    viscosity * plate_velocity / thickness
}

pub fn fault_slip_rate(tectonic_stress: f64, friction_coefficient: f64, normal_stress: f64) -> f64 {
    let coulomb = tectonic_stress - friction_coefficient * normal_stress;
    coulomb.max(0.0)
}

pub fn weathering_rate(
    rate_constant: f64,
    activation_energy: f64,
    temperature: f64,
    precipitation_rate: f64,
) -> f64 {
    rate_constant
        * (-activation_energy / (R_GAS * temperature)).exp()
        * precipitation_rate.powf(0.5)
}

pub fn soil_production_rate(bare_rate: f64, soil_thickness: f64, characteristic_depth: f64) -> f64 {
    bare_rate * (-soil_thickness / characteristic_depth).exp()
}

pub fn landscape_diffusion(diffusivity: f64, curvature: f64) -> f64 {
    diffusivity * curvature
}

pub fn flexural_wavelength(
    elastic_thickness: f64,
    youngs_modulus: f64,
    poisson: f64,
    density_contrast: f64,
    g_surface: f64,
) -> f64 {
    let d = youngs_modulus * elastic_thickness.powi(3) / (12.0 * (1.0 - poisson.powi(2)));
    2.0 * std::f64::consts::PI * (d / (density_contrast * g_surface)).powf(0.25)
}
