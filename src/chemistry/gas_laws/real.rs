use crate::constants::R_GAS;

pub fn van_der_waals_pressure(n: f64, t: f64, v: f64, a: f64, b: f64) -> f64 {
    n * R_GAS * t / (v - n * b).max(1e-30) - a * n * n / (v * v).max(1e-30)
}

pub fn redlich_kwong_pressure(n: f64, t: f64, v: f64, a: f64, b: f64) -> f64 {
    let vm = v / n.max(1e-30);
    R_GAS * t / (vm - b).max(1e-30) - a / (t.sqrt() * vm * (vm + b)).max(1e-30)
}

pub fn compressibility_factor(p: f64, v: f64, n: f64, t: f64) -> f64 {
    p * v / (n * R_GAS * t).max(1e-30)
}

pub fn virial_equation_b(p: f64, t: f64, b: f64) -> f64 {
    R_GAS * t / p.max(1e-30) * (1.0 + b * p / (R_GAS * t))
}

pub fn boyle_temperature(a: f64, b: f64) -> f64 {
    a / (R_GAS * b).max(1e-30)
}

pub fn critical_temperature(a: f64, b: f64) -> f64 {
    8.0 * a / (27.0 * R_GAS * b).max(1e-30)
}

pub fn critical_pressure(a: f64, b: f64) -> f64 {
    a / (27.0 * b * b).max(1e-30)
}

pub fn critical_volume(b: f64) -> f64 {
    3.0 * b
}

pub fn reduced_temperature(t: f64, tc: f64) -> f64 {
    t / tc.max(1e-30)
}

pub fn reduced_pressure(p: f64, pc: f64) -> f64 {
    p / pc.max(1e-30)
}

pub fn peng_robinson_pressure(t: f64, vm: f64, a: f64, b: f64) -> f64 {
    R_GAS * t / (vm - b).max(1e-30) - a / (vm * (vm + b) + b * (vm - b)).max(1e-30)
}

pub fn fugacity_coefficient(z: f64, b_prime: f64, p: f64) -> f64 {
    ((z - 1.0) - (z - b_prime * p).max(1e-30).ln()).exp()
}

pub fn acentric_factor(p_sat: f64, pc: f64) -> f64 {
    -1.0 - (p_sat / pc.max(1e-30)).max(1e-30).log10()
}
