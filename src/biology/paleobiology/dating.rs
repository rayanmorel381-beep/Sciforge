pub fn radiometric_age(parent: f64, daughter: f64, decay_constant: f64) -> f64 {
    (1.0 + daughter / parent).ln() / decay_constant
}

pub fn half_life_to_decay_constant(half_life: f64) -> f64 {
    (2.0_f64).ln() / half_life
}

pub fn carbon14_age(ratio_sample: f64, ratio_modern: f64) -> f64 {
    let half_life = 5730.0;
    let lambda = (2.0_f64).ln() / half_life;
    (ratio_modern / ratio_sample).ln() / lambda
}

pub fn extinction_rate(extinctions: f64, taxa_at_start: f64, interval_myr: f64) -> f64 {
    -(1.0 - extinctions / taxa_at_start).ln() / interval_myr
}

pub fn origination_rate(originations: f64, taxa_at_end: f64, interval_myr: f64) -> f64 {
    -(1.0 - originations / taxa_at_end).ln() / interval_myr
}

pub fn net_diversification_rate(origination: f64, extinction: f64) -> f64 {
    origination - extinction
}

pub fn turnover_rate(origination: f64, extinction: f64) -> f64 {
    (origination + extinction) / 2.0
}

pub fn survivorship_cohort(initial: f64, extinction_rate: f64, t_myr: f64) -> f64 {
    initial * (-extinction_rate * t_myr).exp()
}

pub fn standing_diversity(origination_rate: f64, extinction_rate: f64, d0: f64, t: f64) -> f64 {
    d0 * ((origination_rate - extinction_rate) * t).exp()
}

pub fn taxonomic_rate_sampling_corrected(
    observed_crossers: f64,
    singletons: f64,
    total: f64,
) -> f64 {
    if total <= singletons {
        return 0.0;
    }
    let freq = (total - singletons) / total;
    let raw = -(freq).ln();
    raw * observed_crossers / (total - singletons)
}

pub fn stratigraphic_completeness(gaps_duration: f64, total_duration: f64) -> f64 {
    1.0 - gaps_duration / total_duration
}

pub fn confidence_interval_range(known_range: f64, n_horizons: f64, confidence: f64) -> f64 {
    known_range * (1.0 / (1.0 - confidence).powf(1.0 / (n_horizons - 1.0)) - 1.0)
}

pub fn logistic_diversity(d0: f64, r: f64, k: f64, t: f64) -> f64 {
    k / (1.0 + ((k - d0) / d0) * (-r * t).exp())
}

pub fn recovery_time_after_extinction(
    pre_extinction: f64,
    post_extinction: f64,
    diversification_rate: f64,
) -> f64 {
    if diversification_rate <= 0.0 {
        return f64::INFINITY;
    }
    (pre_extinction / post_extinction).ln() / diversification_rate
}

pub fn signor_lipps_correction(observed_last: f64, sampling_prob: f64, n_taxa: f64) -> f64 {
    observed_last + (1.0 / (sampling_prob * n_taxa))
}

pub fn potassium_argon_age(k40: f64, ar40: f64) -> f64 {
    use crate::constants::{LAMBDA_K40_AR, LAMBDA_K40_TOTAL};
    (1.0 + ar40 * LAMBDA_K40_TOTAL / (LAMBDA_K40_AR * k40)).ln() / LAMBDA_K40_TOTAL
}
