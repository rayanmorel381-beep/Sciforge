use crate::constants::{
    ERF_A1, ERF_A2, ERF_A3, ERF_A4, ERF_A5, ERF_P, STIRLING_LANCZOS_COEFFS, STIRLING_LANCZOS_G,
};
use std::f64::consts::PI;

pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / sigma;
    (-0.5 * z * z).exp() / (sigma * (2.0 * PI).sqrt())
}

pub fn normal_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    0.5 * (1.0 + erf((x - mu) / (sigma * std::f64::consts::SQRT_2)))
}

pub fn erf(x: f64) -> f64 {
    let t = 1.0 / (1.0 + ERF_P * x.abs());
    let poly = t * (ERF_A1 + t * (ERF_A2 + t * (ERF_A3 + t * (ERF_A4 + t * ERF_A5))));
    let result = 1.0 - poly * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}

pub fn erfc(x: f64) -> f64 {
    1.0 - erf(x)
}

pub fn poisson_pmf(k: u64, lambda: f64) -> f64 {
    (-lambda).exp() * lambda.powi(k as i32) / factorial(k) as f64
}

pub fn binomial_pmf(n: u64, k: u64, p: f64) -> f64 {
    binomial_coeff(n, k) as f64 * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

pub fn exponential_pdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    lambda * (-lambda * x).exp()
}

pub fn exponential_cdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    1.0 - (-lambda * x).exp()
}

pub fn chi_squared_pdf(x: f64, k: u32) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let half_k = k as f64 / 2.0;
    x.powf(half_k - 1.0) * (-x / 2.0).exp() / (2.0_f64.powf(half_k) * gamma(half_k))
}

pub fn student_t_pdf(x: f64, nu: f64) -> f64 {
    let coeff = gamma((nu + 1.0) / 2.0) / ((nu * PI).sqrt() * gamma(nu / 2.0));
    coeff * (1.0 + x * x / nu).powf(-(nu + 1.0) / 2.0)
}

pub fn cauchy_pdf(x: f64, x0: f64, gamma_val: f64) -> f64 {
    1.0 / (PI * gamma_val * (1.0 + ((x - x0) / gamma_val).powi(2)))
}

pub fn uniform_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x >= a && x <= b { 1.0 / (b - a) } else { 0.0 }
}

pub fn beta_pdf(x: f64, alpha: f64, beta: f64) -> f64 {
    if x <= 0.0 || x >= 1.0 {
        return 0.0;
    }
    let b = gamma(alpha) * gamma(beta) / gamma(alpha + beta);
    x.powf(alpha - 1.0) * (1.0 - x).powf(beta - 1.0) / b
}

pub fn gamma_pdf(x: f64, shape: f64, rate: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    rate.powf(shape) / gamma(shape) * x.powf(shape - 1.0) * (-rate * x).exp()
}

pub fn gamma(x: f64) -> f64 {
    if x <= 0.5 {
        PI / ((PI * x).sin() * gamma(1.0 - x))
    } else {
        let x = x - 1.0;
        let mut sum = STIRLING_LANCZOS_COEFFS[0];
        for (i, &ci) in STIRLING_LANCZOS_COEFFS.iter().enumerate().skip(1) {
            sum += ci / (x + i as f64);
        }
        let t = x + STIRLING_LANCZOS_G + 0.5;
        (2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * sum
    }
}

pub fn ln_gamma(x: f64) -> f64 {
    gamma(x).ln()
}

pub fn beta_function(a: f64, b: f64) -> f64 {
    gamma(a) * gamma(b) / gamma(a + b)
}

pub fn incomplete_gamma_lower(a: f64, x: f64, terms: usize) -> f64 {
    let mut sum = 0.0;
    let mut term = 1.0 / a;
    sum += term;
    for n in 1..terms {
        term *= x / (a + n as f64);
        sum += term;
    }
    sum * x.powf(a) * (-x).exp()
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn binomial_coeff(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
