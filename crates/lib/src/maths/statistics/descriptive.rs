pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn variance(data: &[f64]) -> f64 {
    let m = mean(data);
    data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64
}

pub fn std_dev(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

pub fn sample_variance(data: &[f64]) -> f64 {
    let m = mean(data);
    data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (data.len() - 1) as f64
}

pub fn sample_std_dev(data: &[f64]) -> f64 {
    sample_variance(data).sqrt()
}

pub fn median(data: &mut [f64]) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len();
    if n.is_multiple_of(2) {
        (data[n / 2 - 1] + data[n / 2]) / 2.0
    } else {
        data[n / 2]
    }
}

pub fn percentile(data: &mut [f64], p: f64) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = p * (data.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    if lo == hi {
        data[lo]
    } else {
        data[lo] + (idx - lo as f64) * (data[hi] - data[lo])
    }
}

pub fn mode(data: &[f64]) -> f64 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut best = sorted[0];
    let mut best_count = 1;
    let mut current = sorted[0];
    let mut current_count = 1;
    for &v in &sorted[1..] {
        if (v - current).abs() < 1e-10 {
            current_count += 1;
        } else {
            if current_count > best_count {
                best = current;
                best_count = current_count;
            }
            current = v;
            current_count = 1;
        }
    }
    if current_count > best_count {
        current
    } else {
        best
    }
}

pub fn skewness(data: &[f64]) -> f64 {
    let m = mean(data);
    let s = std_dev(data);
    let n = data.len() as f64;
    data.iter().map(|x| ((x - m) / s).powi(3)).sum::<f64>() / n
}

pub fn kurtosis(data: &[f64]) -> f64 {
    let m = mean(data);
    let s = std_dev(data);
    let n = data.len() as f64;
    data.iter().map(|x| ((x - m) / s).powi(4)).sum::<f64>() / n - 3.0
}

pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let mx = mean(x);
    let my = mean(y);
    let n = x.len() as f64;
    x.iter()
        .zip(y)
        .map(|(a, b)| (a - mx) * (b - my))
        .sum::<f64>()
        / n
}

pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let cov = covariance(x, y);
    let sx = std_dev(x);
    let sy = std_dev(y);
    if sx < 1e-30 || sy < 1e-30 {
        return 0.0;
    }
    cov / (sx * sy)
}

pub fn weighted_mean(data: &[f64], weights: &[f64]) -> f64 {
    let sum_w: f64 = weights.iter().sum();
    data.iter().zip(weights).map(|(d, w)| d * w).sum::<f64>() / sum_w
}

pub fn geometric_mean(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    data.iter().map(|x| x.ln()).sum::<f64>().div_euclid(n).exp()
        * (data.iter().map(|x| x.ln()).sum::<f64>() % n / n).exp()
}

pub fn harmonic_mean(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    n / data.iter().map(|x| 1.0 / x).sum::<f64>()
}

pub fn entropy(probs: &[f64]) -> f64 {
    -probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|p| p * p.ln())
        .sum::<f64>()
}

pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .filter(|&(&pi, &qi)| pi > 0.0 && qi > 0.0)
        .map(|(&pi, &qi)| pi * (pi / qi).ln())
        .sum()
}
