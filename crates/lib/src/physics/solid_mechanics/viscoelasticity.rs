pub fn maxwell_relaxation(stress_initial_pa: f64, time_s: f64, tau_s: f64) -> f64 {
    stress_initial_pa * (-time_s / tau_s).exp()
}

pub fn kelvin_voigt_creep(stress_pa: f64, e_pa: f64, time_s: f64, tau_s: f64) -> f64 {
    (stress_pa / e_pa) * (1.0 - (-time_s / tau_s).exp())
}

pub fn standard_linear_solid(
    stress_pa: f64,
    e1_pa: f64,
    e2_pa: f64,
    eta_pa_s: f64,
    time_s: f64,
) -> f64 {
    let tau = eta_pa_s / e2_pa;
    (stress_pa / e1_pa) + (stress_pa / e2_pa) * (1.0 - (-time_s / tau).exp())
}

pub fn wlf_shift(t_k: f64, t_ref_k: f64, c1: f64, c2: f64) -> f64 {
    -c1 * (t_k - t_ref_k) / (c2 + (t_k - t_ref_k))
}

pub fn prony_series(
    e_inf_pa: f64,
    coefficients_e_tau: &[(f64, f64)],
    time_s: f64,
) -> f64 {
    let mut e_t = e_inf_pa;
    for &(e_i, tau_i) in coefficients_e_tau {
        e_t += e_i * (-time_s / tau_i).exp();
    }
    e_t
}
