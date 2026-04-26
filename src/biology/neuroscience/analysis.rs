pub fn firing_rate(spikes: &[usize], dt: f64, total_steps: usize) -> f64 {
    if total_steps == 0 {
        return 0.0;
    }
    spikes.len() as f64 / (total_steps as f64 * dt) * 1000.0
}

pub fn interspike_intervals(spikes: &[usize], dt: f64) -> Vec<f64> {
    if spikes.len() < 2 {
        return Vec::new();
    }
    (1..spikes.len())
        .map(|i| (spikes[i] - spikes[i - 1]) as f64 * dt)
        .collect()
}

pub fn coefficient_of_variation(intervals: &[f64]) -> f64 {
    if intervals.is_empty() {
        return 0.0;
    }
    let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
    if mean < 1e-30 {
        return 0.0;
    }
    let var = intervals
        .iter()
        .map(|x| (x - mean) * (x - mean))
        .sum::<f64>()
        / intervals.len() as f64;
    var.sqrt() / mean
}

pub fn fano_factor(spike_counts: &[usize]) -> f64 {
    if spike_counts.is_empty() {
        return 0.0;
    }
    let n = spike_counts.len() as f64;
    let mean = spike_counts.iter().sum::<usize>() as f64 / n;
    if mean < 1e-30 {
        return 0.0;
    }
    let var = spike_counts
        .iter()
        .map(|&c| {
            let d = c as f64 - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    var / mean
}

pub fn spike_count_correlation(spikes_a: &[usize], spikes_b: &[usize]) -> f64 {
    let n = spikes_a.len().min(spikes_b.len()) as f64;
    if n < 2.0 {
        return 0.0;
    }
    let mean_a = spikes_a.iter().sum::<usize>() as f64 / n;
    let mean_b = spikes_b.iter().sum::<usize>() as f64 / n;
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for i in 0..n as usize {
        let da = spikes_a[i] as f64 - mean_a;
        let db = spikes_b[i] as f64 - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    let denom = (var_a * var_b).sqrt();
    if denom < 1e-30 {
        return 0.0;
    }
    cov / denom
}

pub fn cross_correlogram(
    spikes_a: &[f64],
    spikes_b: &[f64],
    bin_width: f64,
    max_lag: f64,
) -> Vec<(f64, usize)> {
    let n_bins = (2.0 * max_lag / bin_width) as usize + 1;
    let mut bins = vec![0usize; n_bins];
    for &ta in spikes_a {
        for &tb in spikes_b {
            let lag = tb - ta;
            if lag.abs() <= max_lag {
                let bin = ((lag + max_lag) / bin_width) as usize;
                if bin < n_bins {
                    bins[bin] += 1;
                }
            }
        }
    }
    bins.iter()
        .enumerate()
        .map(|(i, &count)| {
            let lag = -max_lag + i as f64 * bin_width;
            (lag, count)
        })
        .collect()
}

pub fn local_field_potential_power(signal: &[f64], freq: f64, dt: f64) -> f64 {
    let n = signal.len();
    let omega = 2.0 * std::f64::consts::PI * freq;
    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;
    for (i, &si) in signal.iter().enumerate() {
        let t = i as f64 * dt;
        cos_sum += si * (omega * t).cos();
        sin_sum += si * (omega * t).sin();
    }
    (cos_sum * cos_sum + sin_sum * sin_sum) / (n as f64 * n as f64)
}

pub fn spike_triggered_average(stimulus: &[f64], spike_times: &[usize], window: usize) -> Vec<f64> {
    let mut sta = vec![0.0; window];
    let mut count = 0;
    for &t in spike_times {
        if t >= window && t < stimulus.len() {
            for i in 0..window {
                sta[i] += stimulus[t - window + i];
            }
            count += 1;
        }
    }
    if count > 0 {
        for v in sta.iter_mut() {
            *v /= count as f64;
        }
    }
    sta
}

pub fn burst_detection(isi: &[f64], threshold: f64) -> Vec<(usize, usize)> {
    let mut bursts = Vec::new();
    let mut in_burst = false;
    let mut start = 0;
    for (i, &interval) in isi.iter().enumerate() {
        if interval < threshold {
            if !in_burst {
                start = i;
                in_burst = true;
            }
        } else if in_burst {
            bursts.push((start, i));
            in_burst = false;
        }
    }
    if in_burst {
        bursts.push((start, isi.len()));
    }
    bursts
}

pub fn information_rate(spike_counts: &[usize], stimulus_repeats: usize) -> f64 {
    let n = spike_counts.len();
    if n == 0 || stimulus_repeats == 0 {
        return 0.0;
    }
    let mean = spike_counts.iter().sum::<usize>() as f64 / n as f64;
    let total_var = spike_counts
        .iter()
        .map(|&c| (c as f64 - mean).powi(2))
        .sum::<f64>()
        / n as f64;
    let noise_var = total_var / stimulus_repeats as f64;
    let signal_var = (total_var - noise_var).max(0.0);
    if noise_var < 1e-30 {
        return 0.0;
    }
    0.5 * (1.0 + signal_var / noise_var).ln() / (2.0_f64).ln()
}
