pub fn simple_linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len().min(y.len()) as f64;
    if n < 2.0 {
        return (0.0, 0.0);
    }
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut ss_xy = 0.0;
    let mut ss_xx = 0.0;
    for i in 0..n as usize {
        let dx = x[i] - mx;
        ss_xy += dx * (y[i] - my);
        ss_xx += dx * dx;
    }
    let slope = ss_xy / ss_xx.max(1e-30);
    let intercept = my - slope * mx;
    (intercept, slope)
}

pub fn r_squared(y: &[f64], y_pred: &[f64]) -> f64 {
    let n = y.len().min(y_pred.len());
    let my = y.iter().sum::<f64>() / n as f64;
    let ss_res: f64 = (0..n).map(|i| (y[i] - y_pred[i]).powi(2)).sum();
    let ss_tot: f64 = y.iter().map(|&v| (v - my).powi(2)).sum();
    1.0 - ss_res / ss_tot.max(1e-30)
}

pub fn logistic_regression_probability(beta: &[f64], x: &[f64]) -> f64 {
    let n = beta.len().min(x.len() + 1);
    let mut z = if !beta.is_empty() { beta[0] } else { 0.0 };
    for i in 1..n {
        z += beta[i] * x[i - 1];
    }
    1.0 / (1.0 + (-z).exp())
}

pub fn aic(log_likelihood: f64, k: usize) -> f64 {
    2.0 * k as f64 - 2.0 * log_likelihood
}

pub fn bic(log_likelihood: f64, k: usize, n: usize) -> f64 {
    (n as f64).ln() * k as f64 - 2.0 * log_likelihood
}

pub fn residual_standard_error(residuals: &[f64], p: usize) -> f64 {
    let n = residuals.len();
    if n <= p {
        return 0.0;
    }
    let ss: f64 = residuals.iter().map(|r| r * r).sum();
    (ss / (n - p) as f64).sqrt()
}

pub fn chi_squared_statistic(observed: &[f64], expected: &[f64]) -> f64 {
    observed
        .iter()
        .zip(expected.iter())
        .map(|(&o, &e)| if e > 1e-30 { (o - e).powi(2) / e } else { 0.0 })
        .sum()
}

pub fn welch_t_statistic(m1: f64, m2: f64, s1: f64, s2: f64, n1: usize, n2: usize) -> f64 {
    let se = (s1 * s1 / n1 as f64 + s2 * s2 / n2 as f64).sqrt();
    if se < 1e-30 {
        return 0.0;
    }
    (m1 - m2) / se
}

pub fn mann_whitney_u(ranks_group1: &[f64], n1: usize, n2: usize) -> f64 {
    let r1: f64 = ranks_group1.iter().sum();
    let u1 = r1 - n1 as f64 * (n1 as f64 + 1.0) / 2.0;
    let u2 = (n1 * n2) as f64 - u1;
    u1.min(u2)
}

pub fn bonferroni_correction(p_value: f64, n_tests: usize) -> f64 {
    (p_value * n_tests as f64).min(1.0)
}

pub fn fishers_exact_test_odds(a: usize, b: usize, c: usize, d: usize) -> f64 {
    (a as f64 * d as f64) / (b as f64 * c as f64).max(1e-30)
}

pub fn spearman_rank_correlation(rank_x: &[f64], rank_y: &[f64]) -> f64 {
    let n = rank_x.len().min(rank_y.len()) as f64;
    if n < 2.0 {
        return 0.0;
    }
    let d2_sum: f64 = rank_x
        .iter()
        .zip(rank_y.iter())
        .map(|(&rx, &ry)| (rx - ry).powi(2))
        .sum();
    1.0 - 6.0 * d2_sum / (n * (n * n - 1.0))
}

pub fn power_analysis_two_sample(effect_size: f64, alpha_z: f64, power_z: f64) -> f64 {
    let n = ((alpha_z + power_z) / effect_size).powi(2) * 2.0;
    n.ceil()
}
