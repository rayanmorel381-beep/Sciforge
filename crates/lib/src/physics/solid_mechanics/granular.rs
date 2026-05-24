pub fn mohr_coulomb_shear_strength(cohesion_pa: f64, normal_stress_pa: f64, friction_angle_rad: f64) -> f64 {
    cohesion_pa + normal_stress_pa * friction_angle_rad.tan()
}

pub fn mohr_coulomb_failure(
    sigma_1_pa: f64,
    sigma_3_pa: f64,
    cohesion_pa: f64,
    friction_angle_rad: f64,
) -> f64 {
    let phi = friction_angle_rad;
    let n_phi = (1.0 + phi.sin()) / (1.0 - phi.sin());
    sigma_1_pa - sigma_3_pa * n_phi - 2.0 * cohesion_pa * n_phi.sqrt()
}

pub fn drucker_prager_yield(
    j2: f64,
    pressure_pa: f64,
    alpha: f64,
    k_pa: f64,
) -> f64 {
    j2.sqrt() + alpha * pressure_pa - k_pa
}

pub fn drucker_prager_from_mohr_coulomb(
    cohesion_pa: f64,
    friction_angle_rad: f64,
) -> (f64, f64) {
    let phi = friction_angle_rad;
    let alpha = 2.0 * phi.sin() / (3.0_f64.sqrt() * (3.0 - phi.sin()));
    let k = 6.0 * cohesion_pa * phi.cos() / (3.0_f64.sqrt() * (3.0 - phi.sin()));
    (alpha, k)
}

pub fn cam_clay_yield(p_pa: f64, q_pa: f64, p_c_pa: f64, m_slope: f64) -> f64 {
    q_pa.powi(2) + m_slope.powi(2) * p_pa * (p_pa - p_c_pa)
}

pub fn earth_pressure_active(gamma: f64, depth: f64, friction_angle_rad: f64) -> f64 {
    let phi = friction_angle_rad;
    let ka = (1.0 - phi.sin()) / (1.0 + phi.sin());
    ka * gamma * depth
}

pub fn earth_pressure_passive(gamma: f64, depth: f64, friction_angle_rad: f64) -> f64 {
    let phi = friction_angle_rad;
    let kp = (1.0 + phi.sin()) / (1.0 - phi.sin());
    kp * gamma * depth
}

pub fn bulk_density_packing(particle_density_kg_m3: f64, void_ratio: f64) -> f64 {
    particle_density_kg_m3 / (1.0 + void_ratio)
}

pub fn janssen_silo_stress(
    rho_kg_m3: f64,
    g: f64,
    depth_m: f64,
    radius_m: f64,
    mu_wall: f64,
    k_lateral: f64,
) -> f64 {
    let lambda = 2.0 * mu_wall * k_lateral / radius_m;
    rho_kg_m3 * g / lambda * (1.0 - (-lambda * depth_m).exp())
}
