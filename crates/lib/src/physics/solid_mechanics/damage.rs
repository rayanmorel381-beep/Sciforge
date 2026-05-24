pub fn lemaitre_damage_evolution(
    stress_eq: f64,
    triaxiality: f64,
    plastic_strain_rate: f64,
    s_pa: f64,
    s_exponent: f64,
    young_pa: f64,
) -> f64 {
    let r_v = (2.0 / 3.0) * (1.0 + triaxiality) + 3.0 * (1.0 - 2.0 / 3.0) * triaxiality.powi(2);
    let y = stress_eq.powi(2) * r_v / (2.0 * young_pa);
    (y / s_pa).powf(s_exponent) * plastic_strain_rate
}

pub fn kachanov_creep_damage(stress_pa: f64, a: f64, r: f64, damage: f64) -> f64 {
    a * (stress_pa / (1.0 - damage)).powf(r)
}

pub fn effective_stress(stress_pa: f64, damage: f64) -> f64 {
    stress_pa / (1.0 - damage).max(1e-12)
}

pub fn gurson_yield(
    sigma_eq: f64,
    sigma_y: f64,
    porosity: f64,
    pressure: f64,
    q1: f64,
    q2: f64,
    q3: f64,
) -> f64 {
    (sigma_eq / sigma_y).powi(2)
        + 2.0 * q1 * porosity * (q2 * 3.0 * pressure / (2.0 * sigma_y)).cosh()
        - (1.0 + q3 * porosity * porosity)
}

pub fn johnson_cook_damage_strain(
    triaxiality: f64,
    d1: f64,
    d2: f64,
    d3: f64,
) -> f64 {
    d1 + d2 * (d3 * triaxiality).exp()
}

pub fn cockcroft_latham(stress_max_pa: f64, plastic_strain: f64) -> f64 {
    stress_max_pa.max(0.0) * plastic_strain
}

pub fn weibull_damage_probability(stress_pa: f64, sigma_0_pa: f64, m: f64) -> f64 {
    1.0 - (-(stress_pa / sigma_0_pa).powf(m)).exp()
}
