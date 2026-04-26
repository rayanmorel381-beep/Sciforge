use crate::constants::R_GAS;

pub fn fick_first_law(d: f64, dc_dx: f64) -> f64 {
    -d * dc_dx
}

pub fn fick_second_law_solution(c0: f64, cs: f64, x: f64, d: f64, t: f64) -> f64 {
    cs - (cs - c0) * erf_approx(x / (2.0 * (d * t).sqrt()))
}

fn erf_approx(x: f64) -> f64 {
    let a = 0.3480242;
    let b = -0.0958798;
    let c = 0.7478556;
    let t = 1.0 / (1.0 + 0.47047 * x.abs());
    let result = 1.0 - (a * t + b * t * t + c * t * t * t) * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}

pub fn diffusion_coefficient(d0: f64, q: f64, t: f64) -> f64 {
    d0 * (-q / (R_GAS * t)).exp()
}

pub fn diffusion_length(d: f64, t: f64) -> f64 {
    (2.0 * d * t).sqrt()
}

pub fn interdiffusion_coefficient(d_a: f64, d_b: f64, x_a: f64) -> f64 {
    x_a * d_b + (1.0 - x_a) * d_a
}

pub fn kirkendall_velocity(d_a: f64, d_b: f64, dc_dx: f64, c_total: f64) -> f64 {
    (d_a - d_b) * dc_dx / c_total.max(1e-30)
}

pub fn grain_boundary_diffusion(d_gb: f64, delta: f64, d_l: f64, grain_size: f64) -> f64 {
    d_l + 2.0 * delta * d_gb / grain_size
}

pub fn permeability(d: f64, s: f64) -> f64 {
    d * s
}

pub fn carburization_depth(d: f64, t: f64) -> f64 {
    2.0 * (d * t).sqrt()
}

pub fn mean_free_path(d: f64, n_density: f64) -> f64 {
    1.0 / (std::f64::consts::PI * d * d * n_density * std::f64::consts::SQRT_2)
}
