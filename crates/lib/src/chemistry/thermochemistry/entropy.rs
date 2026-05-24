pub fn entropy_change(q_rev: f64, t: f64) -> f64 {
    q_rev / t
}

pub fn gibbs_free_energy(delta_h: f64, t: f64, delta_s: f64) -> f64 {
    delta_h - t * delta_s
}

pub fn spontaneity_temperature(delta_h: f64, delta_s: f64) -> f64 {
    if delta_s.abs() < 1e-30 {
        return f64::INFINITY;
    }
    delta_h / delta_s
}

pub fn entropy_mixing_ideal(mole_fractions: &[f64]) -> f64 {
    -crate::constants::R_GAS
        * mole_fractions
            .iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| x * x.ln())
            .sum::<f64>()
}

pub fn gibbs_helmholtz(delta_g1: f64, delta_h: f64, t1: f64, t2: f64) -> f64 {
    delta_g1 * t2 / t1 + delta_h * (1.0 - t2 / t1)
}

pub fn trouton_rule_entropy(delta_h_vap: f64, t_boil: f64) -> f64 {
    delta_h_vap / t_boil.max(1e-30)
}

pub fn standard_entropy_of_reaction(
    products_s: &[f64],
    products_coeff: &[f64],
    reactants_s: &[f64],
    reactants_coeff: &[f64],
) -> f64 {
    let sum_p: f64 = products_s
        .iter()
        .zip(products_coeff.iter())
        .map(|(&s, &c)| s * c)
        .sum();
    let sum_r: f64 = reactants_s
        .iter()
        .zip(reactants_coeff.iter())
        .map(|(&s, &c)| s * c)
        .sum();
    sum_p - sum_r
}

pub fn entropy_phase_transition(delta_h: f64, t_transition: f64) -> f64 {
    delta_h / t_transition.max(1e-30)
}

pub fn debye_entropy(t: f64, theta_d: f64) -> f64 {
    let x = theta_d / t.max(1e-30);
    if x < 1e-10 {
        return 0.0;
    }
    3.0 * crate::constants::R_GAS
        * (4.0 / 3.0 * std::f64::consts::PI.powi(4) / x.powi(3) - 3.0 * x.ln())
}

pub fn helmholtz_free_energy(u: f64, t: f64, s: f64) -> f64 {
    u - t * s
}
