pub fn peach_koehler_force(
    stress: [[f64; 3]; 3],
    burgers: [f64; 3],
    line_direction: [f64; 3],
) -> [f64; 3] {
    let sb = [
        stress[0][0] * burgers[0] + stress[0][1] * burgers[1] + stress[0][2] * burgers[2],
        stress[1][0] * burgers[0] + stress[1][1] * burgers[1] + stress[1][2] * burgers[2],
        stress[2][0] * burgers[0] + stress[2][1] * burgers[1] + stress[2][2] * burgers[2],
    ];
    [
        sb[1] * line_direction[2] - sb[2] * line_direction[1],
        sb[2] * line_direction[0] - sb[0] * line_direction[2],
        sb[0] * line_direction[1] - sb[1] * line_direction[0],
    ]
}

pub fn hall_petch(sigma_0_pa: f64, k_y_pa_sqrt_m: f64, grain_diameter_m: f64) -> f64 {
    sigma_0_pa + k_y_pa_sqrt_m / grain_diameter_m.sqrt()
}

pub fn orowan_strengthening(g_pa: f64, b_m: f64, particle_spacing_m: f64) -> f64 {
    g_pa * b_m / particle_spacing_m
}

pub fn taylor_dislocation_strengthening(
    alpha: f64,
    g_pa: f64,
    b_m: f64,
    density_per_m2: f64,
) -> f64 {
    alpha * g_pa * b_m * density_per_m2.sqrt()
}

pub fn screw_dislocation_stress_field(
    g_pa: f64,
    b_m: f64,
    radius_m: f64,
) -> f64 {
    g_pa * b_m / (2.0 * std::f64::consts::PI * radius_m)
}

pub fn edge_dislocation_self_energy(
    g_pa: f64,
    b_m: f64,
    nu: f64,
    r_outer_m: f64,
    r_core_m: f64,
) -> f64 {
    g_pa * b_m * b_m / (4.0 * std::f64::consts::PI * (1.0 - nu)) * (r_outer_m / r_core_m).ln()
}

pub fn screw_dislocation_self_energy(g_pa: f64, b_m: f64, r_outer_m: f64, r_core_m: f64) -> f64 {
    g_pa * b_m * b_m / (4.0 * std::f64::consts::PI) * (r_outer_m / r_core_m).ln()
}

pub fn dislocation_velocity_arrhenius(
    v0_m_per_s: f64,
    activation_energy_j: f64,
    stress_pa: f64,
    activation_volume_m3: f64,
    k_b: f64,
    t_k: f64,
) -> f64 {
    v0_m_per_s * (-(activation_energy_j - stress_pa * activation_volume_m3) / (k_b * t_k)).exp()
}
