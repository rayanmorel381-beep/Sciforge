pub fn gaussian_primitive(alpha: f64, r_sq: f64) -> f64 {
    (-alpha * r_sq).exp()
}

pub fn normalization_s_orbital(alpha: f64) -> f64 {
    (2.0 * alpha / std::f64::consts::PI).powf(0.75)
}

pub fn normalization_p_orbital(alpha: f64) -> f64 {
    (128.0 * alpha.powi(5) / (std::f64::consts::PI * std::f64::consts::PI * std::f64::consts::PI))
        .powf(0.25)
}

pub fn slater_exponent(z_eff: f64, n: f64) -> f64 {
    z_eff / n
}

pub fn boys_function_zero(t: f64) -> f64 {
    if t < 1e-10 {
        return 1.0;
    }
    (std::f64::consts::PI / t).sqrt() * erf_approx(t.sqrt()) / 2.0
}

fn erf_approx(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.3275911 * x.abs());
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    sign * (1.0 - poly * (-x * x).exp())
}

pub fn overlap_integral_1s(alpha1: f64, alpha2: f64, r_sq: f64) -> f64 {
    let gamma = alpha1 + alpha2;
    let factor = alpha1 * alpha2 / gamma;
    (std::f64::consts::PI / gamma).powf(1.5) * (-factor * r_sq).exp()
}

pub fn kinetic_integral_1s(alpha1: f64, alpha2: f64, r_sq: f64) -> f64 {
    let gamma = alpha1 + alpha2;
    let factor = alpha1 * alpha2 / gamma;
    factor
        * (3.0 - 2.0 * factor * r_sq)
        * (std::f64::consts::PI / gamma).powf(1.5)
        * (-factor * r_sq).exp()
}

pub fn sto_ng_coefficients(n: usize) -> Vec<(f64, f64)> {
    match n {
        1 => vec![(1.0, 0.270950)],
        2 => vec![(0.678914, 0.151623), (0.430129, 0.851819)],
        3 => vec![
            (0.444635, 0.109818),
            (0.535328, 0.405771),
            (0.154329, 2.22766),
        ],
        _ => vec![(1.0, 0.270950)],
    }
}

pub fn contracted_gaussian(coeffs: &[(f64, f64)], r_sq: f64) -> f64 {
    coeffs
        .iter()
        .map(|&(c, alpha)| c * normalization_s_orbital(alpha) * gaussian_primitive(alpha, r_sq))
        .sum()
}
