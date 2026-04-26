pub fn lennard_jones(r: f64, epsilon: f64, sigma: f64) -> f64 {
    let sr6 = (sigma / r).powi(6);
    4.0 * epsilon * (sr6 * sr6 - sr6)
}

pub fn lennard_jones_force(r: f64, epsilon: f64, sigma: f64) -> f64 {
    let sr = sigma / r;
    let sr6 = sr.powi(6);
    24.0 * epsilon / r * (2.0 * sr6 * sr6 - sr6)
}

pub fn coulomb_interaction(q1: f64, q2: f64, r: f64, epsilon_r: f64) -> f64 {
    let k = 8.9875e9;
    k * q1 * q2 / (epsilon_r * r * r)
}

pub fn debye_huckel(q: f64, r: f64, kappa: f64, epsilon_r: f64) -> f64 {
    let k = 8.9875e9;
    (k * q / (epsilon_r * r)) * (-kappa * r).exp()
}

pub fn verlet_step(
    positions: &mut [f64],
    velocities: &mut [f64],
    forces: &[f64],
    masses: &[f64],
    dt: f64,
) {
    let n = positions.len();
    for i in 0..n {
        let a = forces[i] / masses[i];
        positions[i] += velocities[i] * dt + 0.5 * a * dt * dt;
        velocities[i] += a * dt;
    }
}

pub fn kinetic_energy(velocities: &[f64], masses: &[f64]) -> f64 {
    let n = velocities.len().min(masses.len());
    let mut ke = 0.0;
    for i in 0..n {
        ke += 0.5 * masses[i] * velocities[i] * velocities[i];
    }
    ke
}

pub fn temperature_from_ke(ke: f64, n_particles: usize, n_dim: usize) -> f64 {
    use crate::constants::K_B;
    let dof = (n_particles * n_dim) as f64;
    2.0 * ke / (dof * K_B)
}

pub fn morse_potential(r: f64, d_e: f64, a: f64, r_e: f64) -> f64 {
    let x = 1.0 - (-a * (r - r_e)).exp();
    d_e * x * x
}

pub fn harmonic_bond(r: f64, k: f64, r0: f64) -> f64 {
    0.5 * k * (r - r0).powi(2)
}

pub fn harmonic_angle(theta: f64, k: f64, theta0: f64) -> f64 {
    0.5 * k * (theta - theta0).powi(2)
}

pub fn dihedral_potential(phi: f64, k: f64, n: f64, delta: f64) -> f64 {
    k * (1.0 + (n * phi - delta).cos())
}

pub fn velocity_verlet_step(
    positions: &mut [f64],
    velocities: &mut [f64],
    forces_old: &[f64],
    forces_new: &[f64],
    masses: &[f64],
    dt: f64,
) {
    let n = positions.len().min(velocities.len()).min(masses.len());
    for i in 0..n {
        let a_old = forces_old[i] / masses[i];
        positions[i] += velocities[i] * dt + 0.5 * a_old * dt * dt;
        let a_new = forces_new[i] / masses[i];
        velocities[i] += 0.5 * (a_old + a_new) * dt;
    }
}

pub fn berendsen_thermostat(
    velocities: &mut [f64],
    current_temp: f64,
    target_temp: f64,
    tau: f64,
    dt: f64,
) {
    if current_temp < 1e-30 {
        return;
    }
    let lambda = (1.0 + dt / tau * (target_temp / current_temp - 1.0)).sqrt();
    for v in velocities.iter_mut() {
        *v *= lambda;
    }
}

pub fn nose_hoover_friction(ke: f64, target_ke: f64, q: f64) -> f64 {
    (ke - target_ke) / q
}

pub fn switching_function(r: f64, r_on: f64, r_off: f64) -> f64 {
    if r <= r_on {
        return 1.0;
    }
    if r >= r_off {
        return 0.0;
    }
    let x = (r_off * r_off - r * r) / (r_off * r_off - r_on * r_on);
    x * x * (3.0 - 2.0 * x)
}

pub fn pair_correlation_bin(
    distances: &[f64],
    r_min: f64,
    r_max: f64,
    n_particles: usize,
    volume: f64,
) -> f64 {
    let count = distances
        .iter()
        .filter(|&&d| d >= r_min && d < r_max)
        .count() as f64;
    let shell_vol = 4.0 / 3.0 * std::f64::consts::PI * (r_max.powi(3) - r_min.powi(3));
    let density = n_particles as f64 / volume;
    count / (n_particles as f64 * shell_vol * density)
}

pub fn pressure_virial(n: usize, volume: f64, temperature: f64, virial_sum: f64) -> f64 {
    use crate::constants::K_B;
    let nkbt = n as f64 * K_B * temperature;
    (nkbt + virial_sum / 3.0) / volume
}

pub fn mean_free_path(density: f64, cross_section: f64) -> f64 {
    1.0 / (density * cross_section).max(1e-30)
}

pub fn born_mayer_repulsion(a: f64, b: f64, r: f64) -> f64 {
    a * (-b * r).exp()
}

pub fn buckingham_potential(a: f64, b: f64, c: f64, r: f64) -> f64 {
    a * (-b * r).exp() - c / r.powi(6)
}
