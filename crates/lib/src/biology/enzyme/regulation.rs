pub fn allosteric_monod_wyman_changeux(substrate: f64, n: f64, l0: f64, kr: f64, kt: f64) -> f64 {
    let alpha_r = substrate / kr;
    let alpha_t = substrate / kt;
    let r_bar = (1.0 + alpha_r).powf(n);
    let t_bar = (1.0 + alpha_t).powf(n);
    let r_frac = r_bar / (r_bar + l0 * t_bar);
    r_frac * alpha_r / (1.0 + alpha_r)
}

pub fn hill_cooperativity(substrate: f64, k_half: f64, n: f64) -> f64 {
    substrate.powf(n) / (k_half.powf(n) + substrate.powf(n))
}

pub fn phosphorylation_switch(
    kinase: f64,
    phosphatase: f64,
    km_kin: f64,
    km_phos: f64,
    vmax_kin: f64,
    vmax_phos: f64,
    total_protein: f64,
) -> f64 {
    let mut phospho: f64 = total_protein * 0.5;
    for _ in 0..200 {
        let unphos = total_protein - phospho;
        let rate_on = vmax_kin * kinase * unphos / (km_kin + unphos);
        let rate_off = vmax_phos * phosphatase * phospho / (km_phos + phospho);
        let new_phospho = phospho + 0.01 * (rate_on - rate_off);
        if (new_phospho - phospho).abs() < 1e-10 {
            return new_phospho.clamp(0.0, total_protein);
        }
        phospho = new_phospho.clamp(0.0, total_protein);
    }
    phospho
}

pub fn zymogen_activation(zymogen: f64, activator: f64, k_act: f64) -> f64 {
    k_act * activator * zymogen
}

pub fn product_inhibition_ordered(
    substrate: f64,
    product: f64,
    vmax_f: f64,
    km: f64,
    kp: f64,
) -> f64 {
    vmax_f * substrate / (km * (1.0 + product / kp) + substrate)
}

pub fn isozyme_total_activity(activities: &[f64], fractions: &[f64]) -> f64 {
    let n = activities.len().min(fractions.len());
    let mut total = 0.0;
    for i in 0..n {
        total += activities[i] * fractions[i];
    }
    total
}

pub fn temperature_activation(rate_ref: f64, ea: f64, t: f64, t_ref: f64) -> f64 {
    rate_ref * (ea / crate::constants::R_GAS * (1.0 / t_ref - 1.0 / t)).exp()
}

pub fn thermal_denaturation(activity: f64, k_denat: f64, t: f64) -> f64 {
    activity * (-k_denat * t).exp()
}

pub fn feedback_inhibition(
    product: f64,
    ki: f64,
    n: f64,
    vmax: f64,
    substrate: f64,
    km: f64,
) -> f64 {
    let inhibition_factor = 1.0 / (1.0 + (product / ki).powf(n));
    vmax * substrate / (km + substrate) * inhibition_factor
}

pub fn cascade_amplification(initial_signal: f64, amplification_factors: &[f64]) -> f64 {
    let mut signal = initial_signal;
    for &factor in amplification_factors {
        signal *= factor;
    }
    signal
}
