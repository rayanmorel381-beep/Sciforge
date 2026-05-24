pub fn jet_lag_resync_time(time_zones_crossed: f64, resync_rate: f64) -> f64 {
    time_zones_crossed / resync_rate
}

pub fn sleep_pressure(wake_duration: f64, buildup_rate: f64, max_pressure: f64) -> f64 {
    max_pressure * (1.0 - (-buildup_rate * wake_duration).exp())
}

pub fn two_process_model(sleep_pressure: f64, circadian_amplitude: f64, phase: f64) -> f64 {
    sleep_pressure + circadian_amplitude * (2.0 * std::f64::consts::PI * phase / 24.0).sin()
}

pub fn photoperiod(latitude_rad: f64, declination_rad: f64) -> f64 {
    let cos_hour_angle = -(latitude_rad.tan() * declination_rad.tan());
    if cos_hour_angle < -1.0 {
        return 24.0;
    }
    if cos_hour_angle > 1.0 {
        return 0.0;
    }
    2.0 * cos_hour_angle.acos() * 12.0 / std::f64::consts::PI
}

pub fn ultradian_rhythm(amplitudes: &[f64], periods: &[f64], t: f64) -> f64 {
    amplitudes
        .iter()
        .zip(periods.iter())
        .map(|(&a, &p)| a * (2.0 * std::f64::consts::PI * t / p).sin())
        .sum()
}

pub fn chronotype_shift(mid_sleep_free: f64, sleep_debt_correction: f64) -> f64 {
    mid_sleep_free - sleep_debt_correction / 2.0
}

pub fn circadian_acrophase(data: &[f64], period: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI / period;
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    for (i, &di) in data.iter().enumerate() {
        let t = i as f64;
        cos_sum += di * (omega * t).cos();
        sin_sum += di * (omega * t).sin();
    }
    sin_sum.atan2(cos_sum)
}

pub fn cosinor_amplitude(data: &[f64], period: f64) -> f64 {
    let n = data.len();
    let omega = 2.0 * std::f64::consts::PI / period;
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    for (i, &di) in data.iter().enumerate() {
        let t = i as f64;
        cos_sum += di * (omega * t).cos();
        sin_sum += di * (omega * t).sin();
    }
    let a = 2.0 * cos_sum / n as f64;
    let b = 2.0 * sin_sum / n as f64;
    (a * a + b * b).sqrt()
}

pub fn social_jet_lag(weekday_midsleep: f64, weekend_midsleep: f64) -> f64 {
    (weekend_midsleep - weekday_midsleep).abs()
}

pub fn mesor(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn sleep_debt(wake_hours: f64, optimal_sleep: f64, actual_sleep: f64) -> f64 {
    let deficit_per_day = optimal_sleep - actual_sleep;
    deficit_per_day * wake_hours / 24.0
}

pub fn circadian_phase_estimate(core_body_temp_min_time: f64) -> f64 {
    (core_body_temp_min_time + 2.0) % 24.0
}

pub fn light_phase_advance(lux: f64, sensitivity: f64, timing_factor: f64) -> f64 {
    sensitivity * lux.ln().max(0.0) * timing_factor
}

pub fn dim_light_melatonin_onset(melatonin_levels: &[f64], threshold: f64) -> Option<usize> {
    melatonin_levels.iter().position(|&m| m >= threshold)
}

pub fn infradian_cycle(base_amplitude: f64, period_days: f64, day: f64) -> f64 {
    base_amplitude * (2.0 * std::f64::consts::PI * day / period_days).sin()
}

pub fn temperature_compensation_q10(rate_t1: f64, rate_t2: f64, t1: f64, t2: f64) -> f64 {
    (rate_t2 / rate_t1).powf(10.0 / (t2 - t1))
}

pub fn masking_effect(endogenous: f64, exogenous_signal: f64, masking_gain: f64) -> f64 {
    endogenous + masking_gain * exogenous_signal
}

pub fn relative_amplitude(max_val: f64, min_val: f64) -> f64 {
    (max_val - min_val) / (max_val + min_val)
}

pub fn interdaily_stability(data: &[f64], period: usize) -> f64 {
    if data.is_empty() || period == 0 {
        return 0.0;
    }
    let n = data.len();
    let grand_mean = data.iter().sum::<f64>() / n as f64;
    let mut hourly_means = vec![0.0; period];
    let mut hourly_counts = vec![0usize; period];
    for (i, &v) in data.iter().enumerate() {
        let bin = i % period;
        hourly_means[bin] += v;
        hourly_counts[bin] += 1;
    }
    for j in 0..period {
        if hourly_counts[j] > 0 {
            hourly_means[j] /= hourly_counts[j] as f64;
        }
    }
    let var_total: f64 = data.iter().map(|&v| (v - grand_mean).powi(2)).sum();
    let var_hourly: f64 = hourly_means
        .iter()
        .zip(hourly_counts.iter())
        .map(|(&m, &c)| c as f64 * (m - grand_mean).powi(2))
        .sum();
    if var_total < 1e-30 {
        return 0.0;
    }
    (n as f64 * var_hourly) / (period as f64 * var_total)
}

pub fn intradaily_variability(data: &[f64]) -> f64 {
    let n = data.len();
    if n < 2 {
        return 0.0;
    }
    let grand_mean = data.iter().sum::<f64>() / n as f64;
    let var_total: f64 = data.iter().map(|&v| (v - grand_mean).powi(2)).sum();
    if var_total < 1e-30 {
        return 0.0;
    }
    let diff_sum: f64 = data.windows(2).map(|w| (w[1] - w[0]).powi(2)).sum();
    (n as f64 * diff_sum) / ((n - 1) as f64 * var_total)
}
