pub fn kepler_period(a: f64, mu: f64) -> f64 {
    2.0 * std::f64::consts::PI * (a.powi(3) / mu).sqrt()
}

pub fn kepler_velocity(mu: f64, r: f64, a: f64) -> f64 {
    (mu * (2.0 / r - 1.0 / a)).sqrt()
}

pub fn circular_velocity(mu: f64, r: f64) -> f64 {
    (mu / r).sqrt()
}

pub fn escape_velocity(mu: f64, r: f64) -> f64 {
    (2.0 * mu / r).sqrt()
}

pub fn vis_viva(mu: f64, r: f64, a: f64) -> f64 {
    mu * (2.0 / r - 1.0 / a)
}

pub fn orbital_energy(mu: f64, a: f64) -> f64 {
    -mu / (2.0 * a)
}

pub fn angular_momentum(mu: f64, a: f64, e: f64) -> f64 {
    (mu * a * (1.0 - e * e)).sqrt()
}

pub fn periapsis(a: f64, e: f64) -> f64 {
    a * (1.0 - e)
}

pub fn apoapsis(a: f64, e: f64) -> f64 {
    a * (1.0 + e)
}

pub fn true_anomaly_to_radius(a: f64, e: f64, theta: f64) -> f64 {
    a * (1.0 - e * e) / (1.0 + e * theta.cos())
}

pub fn hohmann_delta_v(mu: f64, r1: f64, r2: f64) -> f64 {
    let v1 = (mu / r1).sqrt();
    let a_t = (r1 + r2) / 2.0;
    let v_t1 = (mu * (2.0 / r1 - 1.0 / a_t)).sqrt();
    let v_t2 = (mu * (2.0 / r2 - 1.0 / a_t)).sqrt();
    let v2 = (mu / r2).sqrt();
    (v_t1 - v1).abs() + (v2 - v_t2).abs()
}

pub fn sphere_of_influence(a: f64, m_planet: f64, m_star: f64) -> f64 {
    a * (m_planet / m_star).powf(2.0 / 5.0)
}

pub fn roche_limit(r_primary: f64, rho_primary: f64, rho_secondary: f64) -> f64 {
    2.456 * r_primary * (rho_primary / rho_secondary).powf(1.0 / 3.0)
}

pub fn solve_kepler(m: f64, e: f64, tol: f64) -> f64 {
    let mut ea = m;
    loop {
        let delta = (ea - e * ea.sin() - m) / (1.0 - e * ea.cos());
        ea -= delta;
        if delta.abs() < tol {
            break;
        }
    }
    ea
}

pub fn gr_perihelion_precession(mass: f64, a: f64, e: f64) -> f64 {
    use crate::constants::{C, G};
    6.0 * std::f64::consts::PI * G * mass / (a * (1.0 - e * e) * C * C)
}

pub fn j2_value(body: &str) -> Option<f64> {
    use crate::constants::{J2_EARTH, J2_JUPITER, J2_MARS, J2_SATURN, J2_SUN};
    match body.to_ascii_lowercase().as_str() {
        "earth" => Some(J2_EARTH),
        "jupiter" => Some(J2_JUPITER),
        "saturn" => Some(J2_SATURN),
        "mars" => Some(J2_MARS),
        "sun" => Some(J2_SUN),
        _ => None,
    }
}

pub fn j2_nodal_regression(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) -> f64 {
    -1.5 * n * j2 * (r_body / (a * (1.0 - e * e))).powi(2) * i.cos()
}

pub fn j2_apsidal_precession(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) -> f64 {
    0.75 * n * j2 * (r_body / (a * (1.0 - e * e))).powi(2) * (5.0 * i.cos().powi(2) - 1.0)
}

pub fn yoshida4_step(q: &mut [f64], p: &mut [f64], dt: f64, force: &dyn Fn(&[f64]) -> Vec<f64>) {
    use crate::constants::{
        YOSHIDA4_C1, YOSHIDA4_C2, YOSHIDA4_C3, YOSHIDA4_C4, YOSHIDA4_D1, YOSHIDA4_D2, YOSHIDA4_D3,
    };
    let cs = [YOSHIDA4_C1, YOSHIDA4_C2, YOSHIDA4_C3, YOSHIDA4_C4];
    let ds = [YOSHIDA4_D1, YOSHIDA4_D2, YOSHIDA4_D3];

    for i in 0..q.len() {
        q[i] += cs[0] * dt * p[i];
    }
    for k in 0..3 {
        let f = force(q);
        for i in 0..p.len() {
            p[i] += ds[k] * dt * f[i];
        }
        for i in 0..q.len() {
            q[i] += cs[k + 1] * dt * p[i];
        }
    }
}

pub fn yoshida4_weights() -> (f64, f64) {
    use crate::constants::{YOSHIDA4_W0, YOSHIDA4_W1};
    (YOSHIDA4_W0, YOSHIDA4_W1)
}
