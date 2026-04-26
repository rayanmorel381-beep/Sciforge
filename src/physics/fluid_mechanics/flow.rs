pub fn reynolds_number(rho: f64, v: f64, l: f64, mu: f64) -> f64 {
    rho * v * l / mu
}

pub fn bernoulli_pressure(rho: f64, v1: f64, p1: f64, v2: f64) -> f64 {
    p1 + 0.5 * rho * (v1 * v1 - v2 * v2)
}

pub fn bernoulli_height(rho: f64, v1: f64, p1: f64, h1: f64, v2: f64, p2: f64, g: f64) -> f64 {
    h1 + (p1 - p2) / (rho * g) + (v1 * v1 - v2 * v2) / (2.0 * g)
}

pub fn hagen_poiseuille(delta_p: f64, r: f64, l: f64, mu: f64) -> f64 {
    std::f64::consts::PI * r.powi(4) * delta_p / (8.0 * mu * l)
}

pub fn continuity_velocity(a1: f64, v1: f64, a2: f64) -> f64 {
    a1 * v1 / a2
}

pub fn drag_force(cd: f64, rho: f64, v: f64, a: f64) -> f64 {
    0.5 * cd * rho * v * v * a
}

pub fn lift_force(cl: f64, rho: f64, v: f64, a: f64) -> f64 {
    0.5 * cl * rho * v * v * a
}

pub fn stokes_drag(mu: f64, r: f64, v: f64) -> f64 {
    6.0 * std::f64::consts::PI * mu * r * v
}

pub fn terminal_velocity_sphere(rho_p: f64, rho_f: f64, r: f64, mu: f64, g: f64) -> f64 {
    2.0 * r * r * (rho_p - rho_f) * g / (9.0 * mu)
}

pub fn torricelli(g: f64, h: f64) -> f64 {
    (2.0 * g * h).sqrt()
}

pub fn hydraulic_diameter(area: f64, perimeter: f64) -> f64 {
    4.0 * area / perimeter
}

pub fn darcy_weisbach(f: f64, l: f64, d: f64, rho: f64, v: f64) -> f64 {
    f * (l / d) * 0.5 * rho * v * v
}
