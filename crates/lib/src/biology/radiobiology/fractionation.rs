use crate::constants::LN_2;

pub fn bed_biologically_effective_dose(n: f64, d: f64, alpha_beta: f64) -> f64 {
    n * d * (1.0 + d / alpha_beta)
}

pub fn eqd2(n: f64, d: f64, alpha_beta: f64) -> f64 {
    n * d * (d + alpha_beta) / (2.0 + alpha_beta)
}

pub fn tumor_control_probability(n_clonogens: f64, surviving_fraction: f64) -> f64 {
    (-n_clonogens * surviving_fraction).exp()
}

pub fn normal_tissue_complication_probability(dose: f64, td50: f64, gamma50: f64) -> f64 {
    let t = (dose - td50) / (td50 * 0.01).max(1e-30);
    1.0 / (1.0 + (-gamma50 * t * std::f64::consts::PI / (3.0_f64.sqrt())).exp())
}

pub fn incomplete_repair_factor(delta_t: f64, repair_half_time: f64) -> f64 {
    let mu = LN_2 / repair_half_time;
    2.0 / (mu * delta_t).max(1e-30)
        * (1.0 - (1.0 - (-mu * delta_t).exp()) / (mu * delta_t).max(1e-30))
}

pub fn repopulation_dose_equivalent(
    doubling_time: f64,
    treatment_duration: f64,
    alpha: f64,
) -> f64 {
    LN_2 * treatment_duration / (alpha * doubling_time).max(1e-30)
}

pub fn lq_with_repopulation(
    alpha: f64,
    beta: f64,
    dose: f64,
    n_fractions: f64,
    treatment_days: f64,
    tp: f64,
    tk: f64,
) -> f64 {
    let cell_kill = alpha * n_fractions * dose + beta * n_fractions * dose * dose;
    let repop = if treatment_days > tk {
        LN_2 * (treatment_days - tk) / tp
    } else {
        0.0
    };
    cell_kill - repop
}

pub fn therapeutic_ratio(tcp: f64, ntcp: f64) -> f64 {
    tcp * (1.0 - ntcp)
}

pub fn fraction_size_optimization(alpha_beta_tumor: f64, alpha_beta_normal: f64) -> f64 {
    (alpha_beta_tumor / alpha_beta_normal).sqrt()
}

pub fn hyperfractionation_advantage(d_conventional: f64, d_hyper: f64, alpha_beta: f64) -> f64 {
    let bed_conv = d_conventional * (1.0 + d_conventional / alpha_beta);
    let bed_hyper = d_hyper * (1.0 + d_hyper / alpha_beta);
    bed_conv - bed_hyper
}
