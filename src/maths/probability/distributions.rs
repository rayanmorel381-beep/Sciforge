use crate::constants::{
    ERF_A1, ERF_A2, ERF_A3, ERF_A4, ERF_A5, ERF_P, LANCZOS_COEFFS_6, LANCZOS_SERIES_INIT,
    LANCZOS_SQRT_2PI,
};

pub fn uniform_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x >= a && x <= b { 1.0 / (b - a) } else { 0.0 }
}

pub fn uniform_cdf(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        0.0
    } else if x > b {
        1.0
    } else {
        (x - a) / (b - a)
    }
}

pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / sigma;
    (-0.5 * z * z).exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
}

pub fn normal_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    0.5 * (1.0 + erf_approx((x - mu) / (sigma * std::f64::consts::SQRT_2)))
}

pub fn exponential_pdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        lambda * (-lambda * x).exp()
    }
}

pub fn exponential_cdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        1.0 - (-lambda * x).exp()
    }
}

pub fn poisson_pmf(k: u64, lambda: f64) -> f64 {
    let mut log_p = -lambda + k as f64 * lambda.ln();
    for i in 1..=k {
        log_p -= (i as f64).ln();
    }
    log_p.exp()
}

pub fn binomial_pmf(k: u64, n: u64, p: f64) -> f64 {
    let log_c = ln_choose(n, k);
    (log_c + k as f64 * p.ln() + (n - k) as f64 * (1.0 - p).ln()).exp()
}

pub fn geometric_pmf(k: u64, p: f64) -> f64 {
    (1.0 - p).powi((k - 1) as i32) * p
}

pub fn gamma_pdf(x: f64, k: f64, theta: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let log_pdf = (k - 1.0) * x.ln() - x / theta - k * theta.ln() - ln_gamma(k);
    log_pdf.exp()
}

pub fn beta_pdf(x: f64, alpha: f64, beta: f64) -> f64 {
    if x <= 0.0 || x >= 1.0 {
        return 0.0;
    }
    let log_b = ln_gamma(alpha) + ln_gamma(beta) - ln_gamma(alpha + beta);
    ((alpha - 1.0) * x.ln() + (beta - 1.0) * (1.0 - x).ln() - log_b).exp()
}

pub fn cauchy_pdf(x: f64, x0: f64, gamma: f64) -> f64 {
    1.0 / (std::f64::consts::PI * gamma * (1.0 + ((x - x0) / gamma).powi(2)))
}

pub fn chi_squared_pdf(x: f64, k: u32) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let half_k = k as f64 / 2.0;
    let log_pdf = (half_k - 1.0) * x.ln() - x / 2.0 - half_k * 2.0_f64.ln() - ln_gamma(half_k);
    log_pdf.exp()
}

fn erf_approx(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + ERF_P * x);
    let y = 1.0
        - (((((ERF_A5 * t + ERF_A4) * t) + ERF_A3) * t + ERF_A2) * t + ERF_A1) * t * (-x * x).exp();
    sign * y
}

fn ln_gamma(x: f64) -> f64 {
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = LANCZOS_SERIES_INIT;
    for &c in &LANCZOS_COEFFS_6 {
        y += 1.0;
        ser += c / y;
    }
    -tmp + (LANCZOS_SQRT_2PI * ser / x).ln()
}

fn ln_choose(n: u64, k: u64) -> f64 {
    ln_gamma(n as f64 + 1.0) - ln_gamma(k as f64 + 1.0) - ln_gamma((n - k) as f64 + 1.0)
}

pub fn weibull_pdf(x: f64, lambda: f64, k: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    (k / lambda) * (x / lambda).powf(k - 1.0) * (-(x / lambda).powf(k)).exp()
}

pub fn weibull_cdf(x: f64, lambda: f64, k: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        1.0 - (-(x / lambda).powf(k)).exp()
    }
}

pub fn log_normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let z = (x.ln() - mu) / sigma;
    (-0.5 * z * z).exp() / (x * sigma * (2.0 * std::f64::consts::PI).sqrt())
}

pub fn student_t_pdf(x: f64, nu: f64) -> f64 {
    let coeff =
        ln_gamma((nu + 1.0) / 2.0) - ln_gamma(nu / 2.0) - 0.5 * (nu * std::f64::consts::PI).ln();
    (coeff - ((nu + 1.0) / 2.0) * (1.0 + x * x / nu).ln()).exp()
}

pub fn pareto_pdf(x: f64, x_m: f64, alpha: f64) -> f64 {
    if x < x_m {
        0.0
    } else {
        alpha * x_m.powf(alpha) / x.powf(alpha + 1.0)
    }
}

pub fn pareto_cdf(x: f64, x_m: f64, alpha: f64) -> f64 {
    if x < x_m {
        0.0
    } else {
        1.0 - (x_m / x).powf(alpha)
    }
}

pub fn laplace_pdf(x: f64, mu: f64, b: f64) -> f64 {
    (-(x - mu).abs() / b).exp() / (2.0 * b)
}

pub fn rayleigh_pdf(x: f64, sigma: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    x / (sigma * sigma) * (-x * x / (2.0 * sigma * sigma)).exp()
}

pub fn rayleigh_cdf(x: f64, sigma: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        1.0 - (-x * x / (2.0 * sigma * sigma)).exp()
    }
}

pub fn negative_binomial_pmf(k: u64, r: u64, p: f64) -> f64 {
    let log_c = ln_choose(k + r - 1, k);
    (log_c + r as f64 * p.ln() + k as f64 * (1.0 - p).ln()).exp()
}

pub fn hypergeometric_pmf(k: u64, big_n: u64, big_k: u64, n: u64) -> f64 {
    if k > big_k || k > n || n - k > big_n - big_k {
        return 0.0;
    }
    (ln_choose(big_k, k) + ln_choose(big_n - big_k, n - k) - ln_choose(big_n, n)).exp()
}

pub fn entropy_discrete(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|v| **v > 0.0)
        .map(|p| -p * p.ln())
        .sum()
}

pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .filter(|(pi, qi)| **pi > 0.0 && **qi > 0.0)
        .map(|(pi, qi)| pi * (pi / qi).ln())
        .sum()
}
