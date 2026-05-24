pub fn hill_muscle_force(
    f_max_n: f64,
    velocity_m_per_s: f64,
    v_max_m_per_s: f64,
    a_constant_n: f64,
    b_constant_m_per_s: f64,
) -> f64 {
    if velocity_m_per_s >= v_max_m_per_s {
        return 0.0;
    }
    (f_max_n * b_constant_m_per_s - a_constant_n * velocity_m_per_s)
        / (b_constant_m_per_s + velocity_m_per_s)
}

pub fn force_length_relation(
    f_max_n: f64,
    sarcomere_length_m: f64,
    optimal_length_m: f64,
    width_factor: f64,
) -> f64 {
    let x = (sarcomere_length_m - optimal_length_m) / (width_factor * optimal_length_m);
    f_max_n * (-x * x).exp()
}

pub fn passive_tendon_force(
    stiffness_n_per_m: f64,
    extension_m: f64,
    toe_strain: f64,
    rest_length_m: f64,
) -> f64 {
    let strain = extension_m / rest_length_m;
    if strain <= 0.0 {
        0.0
    } else if strain < toe_strain {
        0.5 * stiffness_n_per_m * extension_m * strain / toe_strain
    } else {
        stiffness_n_per_m * extension_m
    }
}

pub fn cross_bridge_power(force_n: f64, velocity_m_per_s: f64) -> f64 {
    force_n * velocity_m_per_s
}
