//! Numeric utilities: scientific / SI formatting, `linspace`, `logspace`,
//! normalization, moving average, and approximate equality.

/// Formats `value` in scientific notation with `precision` decimal places.
pub fn format_scientific(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return "0.0".to_string();
    }
    let exp = value.abs().log10().floor() as i32;
    let mantissa = value / 10f64.powi(exp);
    format!("{mantissa:.precision$}e{exp}")
}

/// Formats `value` with an SI prefix (k, M, G, … or m, µ, n, …).
pub fn format_si(value: f64) -> String {
    let prefixes = [
        (1e24, "Y"),
        (1e21, "Z"),
        (1e18, "E"),
        (1e15, "P"),
        (1e12, "T"),
        (1e9, "G"),
        (1e6, "M"),
        (1e3, "k"),
        (1.0, ""),
        (1e-3, "m"),
        (1e-6, "µ"),
        (1e-9, "n"),
        (1e-12, "p"),
        (1e-15, "f"),
        (1e-18, "a"),
    ];
    let abs = value.abs();
    for &(threshold, prefix) in &prefixes {
        if abs >= threshold {
            return format!("{:.3}{prefix}", value / threshold);
        }
    }
    format_scientific(value, 3)
}

/// Generates `n` evenly spaced values between `start` and `end` inclusive.
pub fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![start];
    }
    let step = (end - start) / (n - 1) as f64;
    (0..n).map(|i| start + i as f64 * step).collect()
}

/// Generates `n` logarithmically spaced values from $10^{\text{start\_exp}}$ to $10^{\text{end\_exp}}$.
pub fn logspace(start_exp: f64, end_exp: f64, n: usize) -> Vec<f64> {
    linspace(start_exp, end_exp, n)
        .into_iter()
        .map(|e| 10f64.powf(e))
        .collect()
}

/// Clamps each element of `data` to `[min, max]` in place.
pub fn clamp_vec(data: &mut [f64], min: f64, max: f64) {
    for v in data.iter_mut() {
        *v = v.clamp(min, max);
    }
}

/// Normalizes `data` to [0, 1] (min-max scaling).
pub fn normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range == 0.0 {
        return vec![0.0; data.len()];
    }
    data.iter().map(|&v| (v - min) / range).collect()
}

/// Returns the cumulative sum of `data`.
pub fn cumulative_sum(data: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len());
    let mut sum = 0.0;
    for &v in data {
        sum += v;
        result.push(sum);
    }
    result
}

/// Computes a centered moving average with a window of `window` elements.
pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || data.is_empty() {
        return data.to_vec();
    }
    let n = data.len();
    (0..n)
        .map(|i| {
            let lo = i.saturating_sub(window / 2);
            let hi = (i + window / 2 + 1).min(n);
            data[lo..hi].iter().sum::<f64>() / (hi - lo) as f64
        })
        .collect()
}

/// Computes the relative error between `computed` and `exact`.
pub fn relative_error(computed: f64, exact: f64) -> f64 {
    if exact == 0.0 {
        return computed.abs();
    }
    ((computed - exact) / exact).abs()
}

/// Returns `true` if `a` and `b` are equal within `tolerance`.
pub fn approx_equal(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}
