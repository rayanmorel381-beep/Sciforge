use crate::constants::{E_CHARGE, HBAR, K_B};

pub fn fermi_energy(n: f64, m_eff: f64) -> f64 {
    HBAR * HBAR / (2.0 * m_eff)
        * (3.0 * std::f64::consts::PI * std::f64::consts::PI * n).powf(2.0 / 3.0)
}

pub fn fermi_dirac(e: f64, ef: f64, t: f64) -> f64 {
    1.0 / (((e - ef) / (K_B * t)).exp() + 1.0)
}

pub fn intrinsic_carrier_concentration(eg: f64, t: f64, nc: f64, nv: f64) -> f64 {
    (nc * nv).sqrt() * (-eg / (2.0 * K_B * t)).exp()
}

pub fn conductivity_semiconductor(n: f64, mu_e: f64, p: f64, mu_h: f64) -> f64 {
    E_CHARGE * (n * mu_e + p * mu_h)
}

pub fn hall_coefficient(n: f64, p: f64, mu_e: f64, mu_h: f64) -> f64 {
    let num = p * mu_h * mu_h - n * mu_e * mu_e;
    let den = E_CHARGE * (p * mu_h + n * mu_e).powi(2);
    num / den.max(1e-50)
}

pub fn drift_velocity(mu: f64, e_field: f64) -> f64 {
    mu * e_field
}

pub fn depletion_width(epsilon: f64, v_bi: f64, na: f64, nd: f64) -> f64 {
    (2.0 * epsilon * v_bi * (na + nd) / (E_CHARGE * na * nd)).sqrt()
}

pub fn built_in_potential(na: f64, nd: f64, ni: f64, t: f64) -> f64 {
    K_B * t / E_CHARGE * (na * nd / (ni * ni)).ln()
}

pub fn diode_current(is: f64, v: f64, t: f64, n: f64) -> f64 {
    is * ((v * E_CHARGE / (n * K_B * t)).exp() - 1.0)
}

pub fn band_gap_temperature(eg0: f64, alpha: f64, beta: f64, t: f64) -> f64 {
    eg0 - alpha * t * t / (t + beta)
}

pub fn doping_ionization(nd: f64, ed: f64, t: f64) -> f64 {
    nd / (1.0 + 0.5 * (ed / (K_B * t)).exp())
}
