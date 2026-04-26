pub fn odds_ratio(a: usize, b: usize, c: usize, d: usize) -> f64 {
    (a as f64 * d as f64) / (b as f64 * c as f64).max(1e-30)
}

pub fn relative_risk(a: usize, b: usize, c: usize, d: usize) -> f64 {
    let risk_exposed = a as f64 / (a + b) as f64;
    let risk_control = c as f64 / (c + d) as f64;
    risk_exposed / risk_control.max(1e-30)
}

pub fn absolute_risk_reduction(risk_control: f64, risk_treatment: f64) -> f64 {
    risk_control - risk_treatment
}

pub fn number_needed_to_treat(arr: f64) -> f64 {
    1.0 / arr.abs().max(1e-30)
}

pub fn sensitivity(tp: usize, fn_count: usize) -> f64 {
    tp as f64 / (tp + fn_count) as f64
}

pub fn specificity(tn: usize, fp: usize) -> f64 {
    tn as f64 / (tn + fp) as f64
}

pub fn positive_predictive_value(tp: usize, fp: usize) -> f64 {
    tp as f64 / (tp + fp) as f64
}

pub fn negative_predictive_value(tn: usize, fn_count: usize) -> f64 {
    tn as f64 / (tn + fn_count) as f64
}

pub fn f1_score(tp: usize, fp: usize, fn_count: usize) -> f64 {
    let prec = tp as f64 / (tp + fp).max(1) as f64;
    let rec = tp as f64 / (tp + fn_count).max(1) as f64;
    if prec + rec < 1e-30 {
        return 0.0;
    }
    2.0 * prec * rec / (prec + rec)
}

pub fn roc_auc(scores: &[(f64, bool)]) -> f64 {
    let mut sorted = scores.to_vec();
    sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let total_pos = sorted.iter().filter(|s| s.1).count() as f64;
    let total_neg = sorted.iter().filter(|s| !s.1).count() as f64;
    if total_pos < 1.0 || total_neg < 1.0 {
        return 0.5;
    }
    let mut auc = 0.0;
    let mut tp = 0.0;
    let mut fp = 0.0;
    let mut prev_fpr = 0.0;
    let mut prev_tpr = 0.0;
    for &(_, label) in &sorted {
        if label {
            tp += 1.0;
        } else {
            fp += 1.0;
        }
        let tpr = tp / total_pos;
        let fpr = fp / total_neg;
        auc += 0.5 * (fpr - prev_fpr) * (tpr + prev_tpr);
        prev_fpr = fpr;
        prev_tpr = tpr;
    }
    auc
}

pub fn cohens_kappa(observed_agreement: f64, expected_agreement: f64) -> f64 {
    if (1.0 - expected_agreement).abs() < 1e-30 {
        return 0.0;
    }
    (observed_agreement - expected_agreement) / (1.0 - expected_agreement)
}

pub fn likelihood_ratio_positive(sensitivity: f64, specificity: f64) -> f64 {
    sensitivity / (1.0 - specificity).max(1e-30)
}

pub fn likelihood_ratio_negative(sensitivity: f64, specificity: f64) -> f64 {
    (1.0 - sensitivity) / specificity.max(1e-30)
}

pub fn diagnostic_odds_ratio(tp: usize, fp: usize, fn_count: usize, tn: usize) -> f64 {
    let lr_pos = (tp as f64 / (tp + fn_count).max(1) as f64)
        / (fp as f64 / (fp + tn).max(1) as f64).max(1e-30);
    let lr_neg = ((fn_count as f64) / (tp + fn_count).max(1) as f64)
        / (tn as f64 / (fp + tn).max(1) as f64).max(1e-30);
    lr_pos / lr_neg.max(1e-30)
}

pub fn youden_index(sensitivity: f64, specificity: f64) -> f64 {
    sensitivity + specificity - 1.0
}

pub fn matthews_correlation_coefficient(tp: usize, tn: usize, fp: usize, fn_count: usize) -> f64 {
    let num = (tp * tn) as f64 - (fp * fn_count) as f64;
    let den =
        ((tp + fp) as f64 * (tp + fn_count) as f64 * (tn + fp) as f64 * (tn + fn_count) as f64)
            .sqrt();
    if den < 1e-30 {
        return 0.0;
    }
    num / den
}

pub fn prevalence_adjusted_ppv(sensitivity: f64, specificity: f64, prevalence: f64) -> f64 {
    let num = sensitivity * prevalence;
    let den = num + (1.0 - specificity) * (1.0 - prevalence);
    num / den.max(1e-30)
}

pub fn sample_size_two_proportions(p1: f64, p2: f64, alpha_z: f64, power_z: f64) -> f64 {
    let p_bar = (p1 + p2) / 2.0;
    let num = (alpha_z * (2.0 * p_bar * (1.0 - p_bar)).sqrt()
        + power_z * (p1 * (1.0 - p1) + p2 * (1.0 - p2)).sqrt())
    .powi(2);
    num / (p1 - p2).powi(2).max(1e-30)
}

pub fn confidence_interval_proportion(p: f64, n: usize, z: f64) -> (f64, f64) {
    let se = (p * (1.0 - p) / n as f64).sqrt();
    (p - z * se, p + z * se)
}

pub fn attributable_risk(risk_exposed: f64, risk_unexposed: f64) -> f64 {
    risk_exposed - risk_unexposed
}

pub fn population_attributable_fraction(
    risk_exposed: f64,
    risk_unexposed: f64,
    prevalence_exposure: f64,
) -> f64 {
    let rr = risk_exposed / risk_unexposed.max(1e-30);
    prevalence_exposure * (rr - 1.0) / (prevalence_exposure * (rr - 1.0) + 1.0)
}
