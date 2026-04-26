pub fn half_life_to_decay_constant(half_life: f64) -> f64 {
    2.0_f64.ln() / half_life.max(1e-30)
}

pub fn decay_constant_to_half_life(lambda: f64) -> f64 {
    2.0_f64.ln() / lambda.max(1e-30)
}

pub fn remaining_nuclei(n0: f64, lambda: f64, t: f64) -> f64 {
    n0 * (-lambda * t).exp()
}

pub fn activity(lambda: f64, n: f64) -> f64 {
    lambda * n
}

pub fn decay_chain_intermediate(n0: f64, lambda1: f64, lambda2: f64, t: f64) -> f64 {
    if (lambda1 - lambda2).abs() < 1e-30 {
        return n0 * lambda1 * t * (-lambda1 * t).exp();
    }
    n0 * lambda1 / (lambda2 - lambda1) * ((-lambda1 * t).exp() - (-lambda2 * t).exp())
}

pub fn specific_activity(lambda: f64, avogadro: f64, molar_mass: f64) -> f64 {
    lambda * avogadro / molar_mass.max(1e-30)
}
