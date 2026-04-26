pub fn decay_remaining(n0: f64, half_life: f64, time: f64) -> f64 {
    n0 * (0.5_f64).powf(time / half_life)
}

pub fn bateman_chain(n0: f64, lambdas: &[f64], time: f64) -> Vec<f64> {
    let n = lambdas.len();
    let mut result = vec![0.0; n];
    result[0] = n0 * (-lambdas[0] * time).exp();
    for (i, ri) in result.iter_mut().enumerate().skip(1) {
        let mut sum = 0.0;
        for j in 0..=i {
            let mut product_num = 1.0;
            for &lk in &lambdas[..i] {
                product_num *= lk;
            }
            let mut product_den = 1.0;
            for k in 0..=i {
                if k != j {
                    let diff = lambdas[k] - lambdas[j];
                    if diff.abs() > 1e-30 {
                        product_den *= diff;
                    }
                }
            }
            if product_den.abs() > 1e-30 {
                sum += product_num * (-lambdas[j] * time).exp() / product_den;
            }
        }
        *ri = n0 * sum;
    }
    result
}

pub fn specific_activity(decay_constant: f64, molar_mass: f64) -> f64 {
    decay_constant * crate::constants::N_A / molar_mass
}

pub fn half_life_from_constant(decay_constant: f64) -> f64 {
    core::f64::consts::LN_2 / decay_constant
}

pub fn mean_lifetime(half_life: f64) -> f64 {
    half_life / core::f64::consts::LN_2
}

pub fn decay_constant_from_half_life(half_life: f64) -> f64 {
    core::f64::consts::LN_2 / half_life
}

pub fn activity_becquerel(n_atoms: f64, half_life: f64) -> f64 {
    n_atoms * core::f64::consts::LN_2 / half_life
}

pub fn activity_curie(n_atoms: f64, half_life: f64) -> f64 {
    activity_becquerel(n_atoms, half_life) / 3.7e10
}

pub fn secular_equilibrium_activity(parent_activity: f64) -> f64 {
    parent_activity
}

pub fn transient_equilibrium_ratio(lambda_parent: f64, lambda_daughter: f64) -> f64 {
    if (lambda_daughter - lambda_parent).abs() < 1e-30 {
        return 1.0;
    }
    lambda_daughter / (lambda_daughter - lambda_parent)
}

pub fn branching_ratio_effective(partial_constants: &[f64]) -> Vec<f64> {
    let total: f64 = partial_constants.iter().sum();
    if total < 1e-30 {
        return vec![0.0; partial_constants.len()];
    }
    partial_constants.iter().map(|&l| l / total).collect()
}

pub fn geiger_nuttall(z: u32, energy_mev: f64) -> f64 {
    let a = -46.83;
    let b = 1.454;
    let c = 28.0;
    let log_half = a + b * z as f64 / energy_mev.sqrt() - c;
    10.0_f64.powf(log_half)
}

pub fn alpha_decay_q(parent_mass_mev: f64, daughter_mass_mev: f64, alpha_mass_mev: f64) -> f64 {
    parent_mass_mev - daughter_mass_mev - alpha_mass_mev
}

pub fn beta_minus_q(parent_mass_amu: f64, daughter_mass_amu: f64) -> f64 {
    (parent_mass_amu - daughter_mass_amu) * crate::constants::AMU_TO_MEV
}

pub fn beta_plus_q(parent_mass_amu: f64, daughter_mass_amu: f64) -> f64 {
    (parent_mass_amu - daughter_mass_amu) * crate::constants::AMU_TO_MEV
        - 2.0 * crate::constants::ELECTRON_REST_MASS_MEV
}

pub fn dose_rate_point_source(activity_bq: f64, gamma_constant: f64, distance_m: f64) -> f64 {
    activity_bq * gamma_constant / (4.0 * std::f64::consts::PI * distance_m * distance_m)
}

pub fn radioactive_dating_age(ratio_daughter_parent: f64, half_life: f64) -> f64 {
    half_life / core::f64::consts::LN_2 * (1.0 + ratio_daughter_parent).ln()
}

pub fn decay_chain_equilibrium_time(lambda_parent: f64, lambda_daughter: f64) -> f64 {
    if (lambda_daughter - lambda_parent).abs() < 1e-30 {
        return f64::INFINITY;
    }
    (lambda_daughter / lambda_parent).ln() / (lambda_daughter - lambda_parent)
}
