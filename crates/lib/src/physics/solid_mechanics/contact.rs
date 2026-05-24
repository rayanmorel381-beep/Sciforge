use std::f64::consts::PI;

pub fn hertz_sphere_sphere_pressure(force_n: f64, r1_m: f64, r2_m: f64, e_star_pa: f64) -> f64 {
    let r = (r1_m * r2_m) / (r1_m + r2_m);
    let a = (3.0 * force_n * r / (4.0 * e_star_pa)).powf(1.0 / 3.0);
    3.0 * force_n / (2.0 * PI * a * a)
}

pub fn jkr_adhesion_pull_off(work_of_adhesion_j_m2: f64, radius_m: f64) -> f64 {
    1.5 * PI * work_of_adhesion_j_m2 * radius_m
}

pub fn dmt_adhesion_pull_off(work_of_adhesion_j_m2: f64, radius_m: f64) -> f64 {
    2.0 * PI * work_of_adhesion_j_m2 * radius_m
}

pub fn mindlin_tangential(
    tangential_force_n: f64,
    normal_force_n: f64,
    mu: f64,
    contact_radius_m: f64,
    g_star_pa: f64,
) -> f64 {
    let q_max = mu * normal_force_n;
    let ratio = (1.0 - tangential_force_n / q_max).max(0.0);
    3.0 * mu * normal_force_n / (16.0 * g_star_pa * contact_radius_m) * (1.0 - ratio.powf(2.0 / 3.0))
}

pub fn cylinder_cylinder_line_contact(
    force_per_length_n_per_m: f64,
    r1_m: f64,
    r2_m: f64,
    e_star_pa: f64,
) -> f64 {
    let r = (r1_m * r2_m) / (r1_m + r2_m);
    let half_width = (4.0 * force_per_length_n_per_m * r / (PI * e_star_pa)).sqrt();
    2.0 * force_per_length_n_per_m / (PI * half_width)
}

pub fn archard_wear_volume(k: f64, normal_force_n: f64, sliding_distance_m: f64, hardness_pa: f64) -> f64 {
    k * normal_force_n * sliding_distance_m / hardness_pa
}

pub fn effective_modulus_e_star(e1_pa: f64, nu1: f64, e2_pa: f64, nu2: f64) -> f64 {
    1.0 / ((1.0 - nu1 * nu1) / e1_pa + (1.0 - nu2 * nu2) / e2_pa)
}
