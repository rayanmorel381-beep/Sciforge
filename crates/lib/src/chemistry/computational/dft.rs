pub fn thomas_fermi_kinetic_energy(density: &[f64], volume_element: f64) -> f64 {
    let cf = 2.871;
    density
        .iter()
        .map(|&rho| cf * rho.powf(5.0 / 3.0) * volume_element)
        .sum()
}

pub fn exchange_energy_lda(density: &[f64], volume_element: f64) -> f64 {
    let cx = -0.7386;
    density
        .iter()
        .map(|&rho| cx * rho.powf(4.0 / 3.0) * volume_element)
        .sum()
}

pub fn hartree_energy(density: &[f64], potential: &[f64], volume_element: f64) -> f64 {
    0.5 * density
        .iter()
        .zip(potential.iter())
        .map(|(&rho, &v)| rho * v * volume_element)
        .sum::<f64>()
}

pub fn nuclear_attraction_energy(z: f64, r: f64) -> f64 {
    -z / r.max(1e-30)
}

pub fn electron_nuclear_energy(
    density: &[f64],
    distances: &[f64],
    z: f64,
    volume_element: f64,
) -> f64 {
    density
        .iter()
        .zip(distances.iter())
        .map(|(&rho, &r)| -z * rho / r.max(1e-30) * volume_element)
        .sum()
}

pub fn xc_potential_lda(density: f64) -> f64 {
    let cx = -0.7386;
    (4.0 / 3.0) * cx * density.powf(1.0 / 3.0)
}

pub fn correlation_energy_vwn(rs: f64) -> f64 {
    let a = 0.0621814;
    let b = 3.72744;
    let c = 12.9352;
    let x0 = -0.10498;
    let x = rs.sqrt();
    let xx = rs + b * x + c;
    let xx0 = x0 * x0 + b * x0 + c;
    a / 2.0
        * ((x / (x - x0)).powi(2).ln() / xx.ln()
            + 2.0 * b / (4.0 * c - b * b).sqrt()
                * ((2.0 * x + b) / (4.0 * c - b * b).sqrt()).atan()
            - b * x0 / xx0
                * (((x - x0).powi(2) / xx).ln()
                    + 2.0 * (b + 2.0 * x0) / (4.0 * c - b * b).sqrt()
                        * ((2.0 * x + b) / (4.0 * c - b * b).sqrt()).atan()))
}

pub fn wigner_seitz_radius(density: f64) -> f64 {
    (3.0 / (4.0 * std::f64::consts::PI * density)).powf(1.0 / 3.0)
}

pub fn kohn_sham_total_energy(
    eigenvalue_sum: f64,
    hartree: f64,
    xc_energy: f64,
    xc_potential_integral: f64,
) -> f64 {
    eigenvalue_sum - hartree + xc_energy - xc_potential_integral
}
