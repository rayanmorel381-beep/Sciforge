pub fn meta_analysis_fixed_effect(effects: &[f64], variances: &[f64]) -> (f64, f64) {
    let n = effects.len().min(variances.len());
    let mut w_sum = 0.0;
    let mut we_sum = 0.0;
    for i in 0..n {
        let w = 1.0 / variances[i].max(1e-30);
        w_sum += w;
        we_sum += w * effects[i];
    }
    let pooled = we_sum / w_sum.max(1e-30);
    let pooled_var = 1.0 / w_sum.max(1e-30);
    (pooled, pooled_var)
}

pub fn cochran_q(effects: &[f64], variances: &[f64]) -> f64 {
    let (pooled, _) = meta_analysis_fixed_effect(effects, variances);
    let n = effects.len().min(variances.len());
    let mut q = 0.0;
    for i in 0..n {
        let w = 1.0 / variances[i].max(1e-30);
        q += w * (effects[i] - pooled).powi(2);
    }
    q
}

pub fn i_squared(q: f64, k: usize) -> f64 {
    if k <= 1 {
        return 0.0;
    }
    let df = (k - 1) as f64;
    ((q - df) / q).max(0.0) * 100.0
}

pub fn tau_squared_dsl(q: f64, k: usize, variances: &[f64]) -> f64 {
    if k <= 1 {
        return 0.0;
    }
    let df = (k - 1) as f64;
    let w_sum: f64 = variances.iter().map(|v| 1.0 / v.max(1e-30)).sum();
    let w2_sum: f64 = variances.iter().map(|v| (1.0 / v.max(1e-30)).powi(2)).sum();
    let c = w_sum - w2_sum / w_sum;
    ((q - df) / c).max(0.0)
}

pub fn meta_analysis_random_effects(effects: &[f64], variances: &[f64], tau2: f64) -> (f64, f64) {
    let n = effects.len().min(variances.len());
    let mut w_sum = 0.0;
    let mut we_sum = 0.0;
    for i in 0..n {
        let w = 1.0 / (variances[i] + tau2).max(1e-30);
        w_sum += w;
        we_sum += w * effects[i];
    }
    let pooled = we_sum / w_sum.max(1e-30);
    let pooled_var = 1.0 / w_sum.max(1e-30);
    (pooled, pooled_var)
}

pub fn funnel_plot_asymmetry(effects: &[f64], se: &[f64]) -> f64 {
    let n = effects.len().min(se.len());
    if n < 3 {
        return 0.0;
    }
    let mean_effect: f64 = effects.iter().sum::<f64>() / n as f64;
    let mut rank_corr = 0.0;
    for i in 0..n {
        rank_corr += (effects[i] - mean_effect) * se[i];
    }
    rank_corr / n as f64
}

pub fn trim_and_fill(effects: &[f64]) -> (f64, usize) {
    let n = effects.len();
    if n < 3 {
        return (effects.iter().sum::<f64>() / n.max(1) as f64, 0);
    }
    let mean = effects.iter().sum::<f64>() / n as f64;
    let mut deviations: Vec<f64> = effects.iter().map(|&e| e - mean).collect();
    deviations.sort_by(|a, b| {
        a.abs()
            .partial_cmp(&b.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let right_count = deviations.iter().filter(|&&d| d > 0.0).count();
    let left_count = deviations.iter().filter(|&&d| d < 0.0).count();
    let missing = right_count.abs_diff(left_count);
    let adjusted_mean = mean - (right_count as f64 - left_count as f64) * 0.01;
    (adjusted_mean, missing)
}

pub fn fail_safe_n(effects: &[f64], variances: &[f64], alpha_z: f64) -> f64 {
    let (pooled, pooled_var) = meta_analysis_fixed_effect(effects, variances);
    let z = pooled / pooled_var.sqrt().max(1e-30);
    let k = effects.len() as f64;
    (k / alpha_z.powi(2)) * (z * z - alpha_z * alpha_z)
}

pub fn prediction_interval(pooled: f64, tau2: f64, se_pooled: f64, k: usize) -> (f64, f64) {
    let t_crit = 1.96;
    let pi_se = (tau2 + se_pooled * se_pooled).sqrt();
    let extra = if k > 2 {
        (k as f64 / (k as f64 - 2.0)).sqrt()
    } else {
        2.0
    };
    (
        pooled - t_crit * pi_se * extra,
        pooled + t_crit * pi_se * extra,
    )
}

pub fn egger_regression(effects: &[f64], se: &[f64]) -> (f64, f64) {
    let n = effects.len().min(se.len());
    if n < 2 {
        return (0.0, 0.0);
    }
    let precision: Vec<f64> = se.iter().map(|&s| 1.0 / s.max(1e-30)).collect();
    let standardized: Vec<f64> = (0..n).map(|i| effects[i] / se[i].max(1e-30)).collect();
    let mean_x = precision.iter().take(n).sum::<f64>() / n as f64;
    let mean_y = standardized.iter().take(n).sum::<f64>() / n as f64;
    let mut ss_xx = 0.0;
    let mut ss_xy = 0.0;
    for i in 0..n {
        let dx = precision[i] - mean_x;
        ss_xx += dx * dx;
        ss_xy += dx * (standardized[i] - mean_y);
    }
    let slope = ss_xy / ss_xx.max(1e-30);
    let intercept = mean_y - slope * mean_x;
    (intercept, slope)
}

pub fn cumulative_meta_analysis(effects: &[f64], variances: &[f64]) -> Vec<(f64, f64)> {
    let n = effects.len().min(variances.len());
    let mut result = Vec::with_capacity(n);
    let mut w_sum = 0.0;
    let mut we_sum = 0.0;
    for i in 0..n {
        let w = 1.0 / variances[i].max(1e-30);
        w_sum += w;
        we_sum += w * effects[i];
        let pooled = we_sum / w_sum.max(1e-30);
        let pooled_var = 1.0 / w_sum.max(1e-30);
        result.push((pooled, pooled_var));
    }
    result
}

pub fn influence_analysis(effects: &[f64], variances: &[f64]) -> Vec<f64> {
    let n = effects.len().min(variances.len());
    let mut leave_one_out = Vec::with_capacity(n);
    for exclude in 0..n {
        let mut w_sum = 0.0;
        let mut we_sum = 0.0;
        for i in 0..n {
            if i == exclude {
                continue;
            }
            let w = 1.0 / variances[i].max(1e-30);
            w_sum += w;
            we_sum += w * effects[i];
        }
        leave_one_out.push(we_sum / w_sum.max(1e-30));
    }
    leave_one_out
}

pub fn h_squared(q: f64, k: usize) -> f64 {
    if k <= 1 {
        return 1.0;
    }
    let df = (k - 1) as f64;
    (q / df).max(1.0)
}

pub fn meta_regression_slope(effects: &[f64], variances: &[f64], covariate: &[f64]) -> f64 {
    let n = effects.len().min(variances.len()).min(covariate.len());
    let mut w_sum = 0.0;
    let mut wx_sum = 0.0;
    let mut wy_sum = 0.0;
    let mut wxx_sum = 0.0;
    let mut wxy_sum = 0.0;
    for i in 0..n {
        let w = 1.0 / variances[i].max(1e-30);
        w_sum += w;
        wx_sum += w * covariate[i];
        wy_sum += w * effects[i];
        wxx_sum += w * covariate[i] * covariate[i];
        wxy_sum += w * covariate[i] * effects[i];
    }
    let denom = w_sum * wxx_sum - wx_sum * wx_sum;
    if denom.abs() < 1e-30 {
        return 0.0;
    }
    (w_sum * wxy_sum - wx_sum * wy_sum) / denom
}
