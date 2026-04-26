use crate::constants::{EPSILON_0, K_COULOMB, MU_0};

pub fn electric_field_point_charge(q: f64, r: [f64; 3]) -> [f64; 3] {
    let r_mag = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    if r_mag < 1e-30 {
        return [0.0; 3];
    }
    let e_mag = K_COULOMB * q / (r_mag * r_mag);
    [
        e_mag * r[0] / r_mag,
        e_mag * r[1] / r_mag,
        e_mag * r[2] / r_mag,
    ]
}

pub fn electric_potential_point(q: f64, r: f64) -> f64 {
    K_COULOMB * q / r
}

pub fn magnetic_field_wire(current: f64, r: f64) -> f64 {
    MU_0 * current / (2.0 * std::f64::consts::PI * r)
}

pub fn magnetic_field_solenoid(n_per_length: f64, current: f64) -> f64 {
    MU_0 * n_per_length * current
}

pub fn magnetic_field_loop(current: f64, radius: f64, z: f64) -> f64 {
    MU_0 * current * radius * radius / (2.0 * (radius * radius + z * z).powf(1.5))
}

pub fn biot_savart_segment(current: f64, dl: [f64; 3], r: [f64; 3]) -> [f64; 3] {
    let r_mag = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    if r_mag < 1e-30 {
        return [0.0; 3];
    }
    let cross = [
        dl[1] * r[2] - dl[2] * r[1],
        dl[2] * r[0] - dl[0] * r[2],
        dl[0] * r[1] - dl[1] * r[0],
    ];
    let factor = MU_0 * current / (4.0 * std::f64::consts::PI * r_mag.powi(3));
    [factor * cross[0], factor * cross[1], factor * cross[2]]
}

pub fn lorentz_force(q: f64, v: [f64; 3], e: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let vxb = [
        v[1] * b[2] - v[2] * b[1],
        v[2] * b[0] - v[0] * b[2],
        v[0] * b[1] - v[1] * b[0],
    ];
    [
        q * (e[0] + vxb[0]),
        q * (e[1] + vxb[1]),
        q * (e[2] + vxb[2]),
    ]
}

pub fn poynting_vector(e: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let inv_mu0 = 1.0 / MU_0;
    [
        inv_mu0 * (e[1] * b[2] - e[2] * b[1]),
        inv_mu0 * (e[2] * b[0] - e[0] * b[2]),
        inv_mu0 * (e[0] * b[1] - e[1] * b[0]),
    ]
}

pub fn energy_density_em(e: [f64; 3], b: [f64; 3]) -> f64 {
    let e_sq = e[0] * e[0] + e[1] * e[1] + e[2] * e[2];
    let b_sq = b[0] * b[0] + b[1] * b[1] + b[2] * b[2];
    0.5 * EPSILON_0 * e_sq + 0.5 * b_sq / MU_0
}

pub fn electric_dipole_field(p: [f64; 3], r: [f64; 3]) -> [f64; 3] {
    let r_mag = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    if r_mag < 1e-30 {
        return [0.0; 3];
    }
    let r5 = r_mag.powi(5);
    let r3 = r_mag.powi(3);
    let p_dot_r = p[0] * r[0] + p[1] * r[1] + p[2] * r[2];
    let factor = 1.0 / (4.0 * std::f64::consts::PI * EPSILON_0);
    [
        factor * (3.0 * p_dot_r * r[0] / r5 - p[0] / r3),
        factor * (3.0 * p_dot_r * r[1] / r5 - p[1] / r3),
        factor * (3.0 * p_dot_r * r[2] / r5 - p[2] / r3),
    ]
}

pub fn magnetic_dipole_field(m: [f64; 3], r: [f64; 3]) -> [f64; 3] {
    let r_mag = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    if r_mag < 1e-30 {
        return [0.0; 3];
    }
    let r5 = r_mag.powi(5);
    let r3 = r_mag.powi(3);
    let m_dot_r = m[0] * r[0] + m[1] * r[1] + m[2] * r[2];
    let factor = MU_0 / (4.0 * std::f64::consts::PI);
    [
        factor * (3.0 * m_dot_r * r[0] / r5 - m[0] / r3),
        factor * (3.0 * m_dot_r * r[1] / r5 - m[1] / r3),
        factor * (3.0 * m_dot_r * r[2] / r5 - m[2] / r3),
    ]
}

pub fn capacitance_parallel_plate(area: f64, distance: f64, epsilon_r: f64) -> f64 {
    epsilon_r * EPSILON_0 * area / distance
}

pub fn inductance_solenoid(n_turns: f64, length: f64, area: f64) -> f64 {
    MU_0 * n_turns * n_turns * area / length
}

pub fn cyclotron_frequency(charge: f64, mass: f64, b: f64) -> f64 {
    charge.abs() * b / mass
}

pub fn larmor_radius(mass: f64, v_perp: f64, charge: f64, b: f64) -> f64 {
    mass * v_perp / (charge.abs() * b)
}

pub fn plasma_frequency(number_density: f64, mass: f64, charge: f64) -> f64 {
    (number_density * charge * charge / (EPSILON_0 * mass)).sqrt()
}

pub fn debye_length(temperature: f64, number_density: f64, charge: f64) -> f64 {
    use crate::constants::K_B;
    (EPSILON_0 * K_B * temperature / (number_density * charge * charge)).sqrt()
}
