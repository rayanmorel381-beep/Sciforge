pub fn worm_like_chain(l: f64, lp: f64, lc: f64) -> f64 {
    use crate::constants::K_B;
    let t = 300.0;
    let x = l / lc;
    if x >= 1.0 {
        return f64::INFINITY;
    }
    (K_B * t / lp) * (0.25 / ((1.0 - x).powi(2)) - 0.25 + x)
}

pub fn freely_jointed_chain(l: f64, n: usize, b: f64) -> f64 {
    use crate::constants::K_B;
    let t = 300.0;
    let lc = n as f64 * b;
    let x = l / lc;
    if x.abs() >= 1.0 {
        return f64::INFINITY;
    }
    let inv_langevin = 3.0 * x + 9.0 * x.powi(3) / 5.0 + 297.0 * x.powi(5) / 175.0;
    K_B * t / b * inv_langevin
}

pub fn end_to_end_distance_rms(n: usize, b: f64) -> f64 {
    (n as f64).sqrt() * b
}

pub fn radius_of_gyration(n: usize, b: f64) -> f64 {
    b * (n as f64 / 6.0).sqrt()
}

pub fn persistence_length_from_tangent(cos_theta: f64, segment_length: f64) -> f64 {
    if cos_theta.abs() >= 1.0 {
        return f64::INFINITY;
    }
    -segment_length / cos_theta.ln()
}

pub fn kratky_porod_energy(kappa: f64, ds: f64, curvature: f64) -> f64 {
    0.5 * kappa * curvature * curvature * ds
}

pub fn dna_twist_energy(c_twist: f64, delta_twist: f64, length: f64) -> f64 {
    2.0 * std::f64::consts::PI * std::f64::consts::PI * c_twist * delta_twist * delta_twist / length
}

pub fn stokes_einstein_diffusion(t: f64, viscosity: f64, radius: f64) -> f64 {
    use crate::constants::K_B;
    K_B * t / (6.0 * std::f64::consts::PI * viscosity * radius)
}

pub fn mean_squared_displacement(d: f64, t: f64, n_dim: usize) -> f64 {
    2.0 * n_dim as f64 * d * t
}

pub fn sedimentation_coefficient(
    mass: f64,
    partial_specific_vol: f64,
    rho_solvent: f64,
    friction: f64,
) -> f64 {
    mass * (1.0 - partial_specific_vol * rho_solvent) / friction
}

pub fn flory_radius(n: usize, b: f64, nu: f64) -> f64 {
    b * (n as f64).powf(nu)
}

pub fn kuhn_length(persistence_length: f64) -> f64 {
    2.0 * persistence_length
}

pub fn contour_length(n: usize, b: f64) -> f64 {
    n as f64 * b
}

pub fn extensible_wlc(force: f64, lp: f64, lc: f64, stretch_modulus: f64, t: f64) -> f64 {
    use crate::constants::K_B;
    let kbt = K_B * t;
    let x = 1.0 - (kbt / (force * lp)).sqrt() / 2.0 + force / stretch_modulus;
    x * lc
}

pub fn odijk_deflection_length(lp: f64, d: f64) -> f64 {
    (lp * d * d).cbrt()
}

pub fn blob_size(kbt: f64, force: f64) -> f64 {
    kbt / force.max(1e-30)
}

pub fn zimm_relaxation_time(viscosity: f64, rg: f64, kbt: f64) -> f64 {
    viscosity * rg.powi(3) / kbt.max(1e-30)
}

pub fn rouse_relaxation_time(friction: f64, n: usize, b: f64, kbt: f64) -> f64 {
    friction * (n as f64 * b).powi(2)
        / (3.0 * std::f64::consts::PI * std::f64::consts::PI * kbt.max(1e-30))
}

pub fn intrinsic_viscosity(rg: f64, mw: f64) -> f64 {
    let phi = 2.5e23;
    phi * rg.powi(3) / mw.max(1e-30)
}

pub fn overlap_concentration(mw: f64, rg: f64) -> f64 {
    mw / (crate::constants::N_A * (4.0 / 3.0) * std::f64::consts::PI * rg.powi(3))
}

pub fn debye_scattering(q: f64, rg: f64) -> f64 {
    let x = (q * rg).powi(2);
    if x < 1e-10 {
        return 1.0;
    }
    2.0 * (x - 1.0 + (-x).exp()) / (x * x)
}
