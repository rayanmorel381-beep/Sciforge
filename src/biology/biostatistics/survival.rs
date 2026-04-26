pub fn kaplan_meier(times: &[f64], events: &[bool]) -> Vec<(f64, f64)> {
    let n = times.len().min(events.len());
    let mut data: Vec<(f64, bool)> = (0..n).map(|i| (times[i], events[i])).collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut survival = 1.0;
    let mut at_risk = n;
    let mut curve = Vec::new();
    curve.push((0.0, 1.0));
    for &(t, event) in &data {
        if event {
            survival *= 1.0 - 1.0 / at_risk as f64;
            curve.push((t, survival));
        }
        at_risk -= 1;
    }
    curve
}

pub fn log_rank_statistic(
    times1: &[f64],
    events1: &[bool],
    times2: &[f64],
    events2: &[bool],
) -> f64 {
    let mut all_times: Vec<f64> = Vec::new();
    for &t in times1 {
        all_times.push(t);
    }
    for &t in times2 {
        all_times.push(t);
    }
    all_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    all_times.dedup();

    let mut o1 = 0.0;
    let mut e1 = 0.0;
    let mut var = 0.0;

    for &t in &all_times {
        let d1 = events1
            .iter()
            .zip(times1)
            .filter(|&(&e, &ti)| e && (ti - t).abs() < 1e-12)
            .count() as f64;
        let d2 = events2
            .iter()
            .zip(times2)
            .filter(|&(&e, &ti)| e && (ti - t).abs() < 1e-12)
            .count() as f64;
        let n1 = times1.iter().filter(|&&ti| ti >= t).count() as f64;
        let n2 = times2.iter().filter(|&&ti| ti >= t).count() as f64;
        let d = d1 + d2;
        let n = n1 + n2;
        if n > 0.0 {
            let expected = n1 * d / n;
            o1 += d1;
            e1 += expected;
            if n > 1.0 {
                var += expected * (1.0 - n1 / n) * (n - d) / (n - 1.0);
            }
        }
    }
    if var < 1e-30 {
        return 0.0;
    }
    (o1 - e1).powi(2) / var
}

pub fn hazard_ratio(
    events_treatment: usize,
    time_treatment: f64,
    events_control: usize,
    time_control: f64,
) -> f64 {
    let rate_t = events_treatment as f64 / time_treatment;
    let rate_c = events_control as f64 / time_control;
    rate_t / rate_c.max(1e-30)
}

pub fn median_survival(curve: &[(f64, f64)]) -> f64 {
    for i in 1..curve.len() {
        if curve[i].1 <= 0.5 {
            let t0 = curve[i - 1].0;
            let t1 = curve[i].0;
            let s0 = curve[i - 1].1;
            let s1 = curve[i].1;
            if (s0 - s1).abs() < 1e-30 {
                return t1;
            }
            return t0 + (0.5 - s0) * (t1 - t0) / (s1 - s0);
        }
    }
    f64::INFINITY
}

pub fn nelson_aalen(times: &[f64], events: &[bool]) -> Vec<(f64, f64)> {
    let n = times.len().min(events.len());
    let mut data: Vec<(f64, bool)> = (0..n).map(|i| (times[i], events[i])).collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut cumulative_hazard = 0.0;
    let mut at_risk = n;
    let mut estimate = Vec::new();
    estimate.push((0.0, 0.0));
    for &(t, event) in &data {
        if event {
            cumulative_hazard += 1.0 / at_risk as f64;
            estimate.push((t, cumulative_hazard));
        }
        at_risk -= 1;
    }
    estimate
}

pub fn exponential_survival(lambda: f64, t: f64) -> f64 {
    (-lambda * t).exp()
}

pub fn weibull_survival(lambda: f64, k: f64, t: f64) -> f64 {
    (-(lambda * t).powf(k)).exp()
}

pub fn restricted_mean_survival_time(curve: &[(f64, f64)], t_max: f64) -> f64 {
    let mut area = 0.0;
    for i in 1..curve.len() {
        if curve[i].0 > t_max {
            break;
        }
        let dt = curve[i].0 - curve[i - 1].0;
        area += curve[i - 1].1 * dt;
    }
    area
}

pub fn greenwood_variance(curve: &[(f64, f64)], at_risk: &[usize], events: &[usize]) -> Vec<f64> {
    let mut variances = Vec::with_capacity(curve.len());
    let mut sum = 0.0;
    variances.push(0.0);
    for i in 1..curve.len().min(at_risk.len()).min(events.len()) {
        let ni = at_risk[i] as f64;
        let di = events[i] as f64;
        if ni > 0.0 && ni > di {
            sum += di / (ni * (ni - di));
        }
        let s = curve[i].1;
        variances.push(s * s * sum);
    }
    variances
}

pub fn cumulative_incidence(times: &[f64], events: &[bool], competing: &[bool]) -> Vec<(f64, f64)> {
    let n = times.len().min(events.len()).min(competing.len());
    let mut data: Vec<(f64, bool, bool)> = (0..n)
        .map(|i| (times[i], events[i], competing[i]))
        .collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut survival = 1.0;
    let mut cumulative = 0.0;
    let mut at_risk = n;
    let mut result = vec![(0.0, 0.0)];
    for &(t, event, comp) in &data {
        if event {
            cumulative += survival / at_risk as f64;
            result.push((t, cumulative));
        }
        if event || comp {
            survival *= 1.0 - 1.0 / at_risk as f64;
        }
        at_risk -= 1;
    }
    result
}

pub fn life_table(
    age_groups: &[(f64, f64)],
    deaths: &[f64],
    population: &[f64],
) -> Vec<(f64, f64, f64)> {
    let mut lx = 1.0;
    let mut table = Vec::new();
    for i in 0..age_groups.len().min(deaths.len()).min(population.len()) {
        let qx = deaths[i] / population[i].max(1.0);
        let width = age_groups[i].1 - age_groups[i].0;
        let dx = lx * qx;
        let big_lx = lx * width * (1.0 - qx / 2.0);
        table.push((lx, dx, big_lx));
        lx -= dx;
        lx = lx.max(0.0);
    }
    table
}

pub fn log_logistic_survival(alpha: f64, beta: f64, t: f64) -> f64 {
    1.0 / (1.0 + (t / alpha).powf(beta))
}

pub fn gompertz_survival(alpha: f64, beta: f64, t: f64) -> f64 {
    (-(alpha / beta) * ((beta * t).exp() - 1.0)).exp()
}

pub fn cox_partial_likelihood_contribution(beta_x: f64, risk_set_sum: f64) -> f64 {
    beta_x.exp() / risk_set_sum.max(1e-30)
}

pub fn breslow_cumulative_hazard(event_times: &[f64], risk_set_sums: &[f64]) -> Vec<(f64, f64)> {
    let n = event_times.len().min(risk_set_sums.len());
    let mut h0 = 0.0;
    let mut result = vec![(0.0, 0.0)];
    for i in 0..n {
        h0 += 1.0 / risk_set_sums[i].max(1e-30);
        result.push((event_times[i], h0));
    }
    result
}

pub fn survival_from_hazard(cumulative_hazard: f64) -> f64 {
    (-cumulative_hazard).exp()
}

pub fn conditional_survival(s_t: f64, s_t_plus_x: f64) -> f64 {
    if s_t < 1e-30 {
        return 0.0;
    }
    s_t_plus_x / s_t
}

pub fn cure_fraction_model(cure_rate: f64, lambda: f64, t: f64) -> f64 {
    cure_rate + (1.0 - cure_rate) * (-lambda * t).exp()
}

pub fn aalen_johansen(times: &[f64], events: &[u8], n_causes: usize) -> Vec<Vec<(f64, f64)>> {
    let n = times.len().min(events.len());
    let mut data: Vec<(f64, u8)> = (0..n).map(|i| (times[i], events[i])).collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut result: Vec<Vec<(f64, f64)>> = (0..n_causes).map(|_| vec![(0.0, 0.0)]).collect();
    let mut survival = 1.0;
    let mut at_risk = n;
    for &(t, cause) in &data {
        if cause > 0 && (cause as usize) <= n_causes {
            let ci_increment = survival / at_risk.max(1) as f64;
            let idx = (cause as usize) - 1;
            let prev = result[idx].last().map_or(0.0, |v| v.1);
            result[idx].push((t, prev + ci_increment));
            survival *= 1.0 - 1.0 / at_risk.max(1) as f64;
        }
        at_risk = at_risk.saturating_sub(1);
    }
    result
}
