pub fn membrane_bending_energy(kappa: f64, curvature: f64, spontaneous_curvature: f64) -> f64 {
    0.5 * kappa * (curvature - spontaneous_curvature).powi(2)
}

pub fn helfrich_energy(kappa: f64, kappa_bar: f64, c1: f64, c2: f64, c0: f64) -> f64 {
    0.5 * kappa * (c1 + c2 - c0).powi(2) + kappa_bar * c1 * c2
}

pub fn membrane_tension(area_strain: f64, stretch_modulus: f64) -> f64 {
    stretch_modulus * area_strain
}

pub fn lipid_diffusion_saffman_delbruck(
    viscosity_membrane: f64,
    viscosity_water: f64,
    membrane_thickness: f64,
    radius: f64,
    t: f64,
) -> f64 {
    use crate::constants::K_B;
    let eta_ratio = viscosity_membrane * membrane_thickness / (viscosity_water * radius);
    K_B * t / (4.0 * std::f64::consts::PI * viscosity_membrane * membrane_thickness)
        * (eta_ratio.ln() - 0.5772)
}

pub fn osmotic_lysis_threshold(
    internal_osmolarity: f64,
    membrane_tension_max: f64,
    radius: f64,
) -> f64 {
    let t = 310.0;
    let pressure_max = 2.0 * membrane_tension_max / radius;
    let delta_c = pressure_max / (crate::constants::N_A * crate::constants::K_B * t);
    internal_osmolarity - delta_c
}

pub fn vesicle_budding_energy(kappa: f64, radius: f64) -> f64 {
    8.0 * std::f64::consts::PI * kappa + 4.0 * std::f64::consts::PI * kappa / radius
}

pub fn flip_flop_rate(activation_energy: f64, t: f64) -> f64 {
    use crate::constants::K_B;
    let k0 = 1e12;
    k0 * (-activation_energy / (K_B * t)).exp()
}

pub fn lateral_pressure_profile(
    depth: f64,
    head_pressure: f64,
    tail_pressure: f64,
    thickness: f64,
) -> f64 {
    let mid = thickness / 2.0;
    if depth < mid {
        head_pressure * (1.0 - depth / mid)
    } else {
        -tail_pressure * (depth - mid) / mid
    }
}

pub fn line_tension_domain(length: f64, lambda: f64) -> f64 {
    lambda * length
}
