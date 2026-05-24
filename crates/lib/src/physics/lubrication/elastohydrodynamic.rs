pub fn dowson_higginson_film_thickness(
    reduced_radius_m: f64,
    reduced_modulus_pa: f64,
    velocity_m_per_s: f64,
    eta0_pa_s: f64,
    pressure_viscosity_coeff_per_pa: f64,
    load_n: f64,
) -> f64 {
    let u = eta0_pa_s * velocity_m_per_s / (reduced_modulus_pa * reduced_radius_m);
    let g = pressure_viscosity_coeff_per_pa * reduced_modulus_pa;
    let w = load_n / (reduced_modulus_pa * reduced_radius_m * reduced_radius_m);
    2.65 * reduced_radius_m * u.powf(0.7) * g.powf(0.54) * w.powf(-0.13)
}

pub fn hamrock_dowson_central_film(
    reduced_radius_m: f64,
    reduced_modulus_pa: f64,
    velocity_m_per_s: f64,
    eta0_pa_s: f64,
    alpha_per_pa: f64,
    load_n: f64,
    ellipticity_k: f64,
) -> f64 {
    let u = eta0_pa_s * velocity_m_per_s / (reduced_modulus_pa * reduced_radius_m);
    let g = alpha_per_pa * reduced_modulus_pa;
    let w = load_n / (reduced_modulus_pa * reduced_radius_m * reduced_radius_m);
    reduced_radius_m
        * 3.63
        * u.powf(0.68)
        * g.powf(0.49)
        * w.powf(-0.073)
        * (1.0 - (-0.68 * ellipticity_k).exp())
}

pub fn pressure_viscosity_barus(eta0_pa_s: f64, alpha_per_pa: f64, pressure_pa: f64) -> f64 {
    eta0_pa_s * (alpha_per_pa * pressure_pa).exp()
}

pub fn pressure_viscosity_roelands(
    eta0_pa_s: f64,
    pressure_pa: f64,
    z_factor: f64,
) -> f64 {
    let p0 = 1.96e8;
    let exponent = (-1.0 + (1.0 + pressure_pa / p0).powf(z_factor))
        * (eta0_pa_s.ln() + 9.67);
    eta0_pa_s * exponent.exp() / eta0_pa_s.powi(0)
}
