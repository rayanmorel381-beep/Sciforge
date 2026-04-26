use crate::constants::C;

pub fn velocity_addition(u: f64, v: f64) -> f64 {
    (u + v) / (1.0 + u * v / (C * C))
}

pub fn velocity_addition_3d(u: [f64; 3], v: f64, dir: [f64; 3]) -> [f64; 3] {
    let g = super::lorentz::gamma_factor(v);
    let u_par: f64 = u[0] * dir[0] + u[1] * dir[1] + u[2] * dir[2];
    let u_perp = [
        u[0] - u_par * dir[0],
        u[1] - u_par * dir[1],
        u[2] - u_par * dir[2],
    ];
    let denom = 1.0 + u_par * v / (C * C);
    let w_par = (u_par + v) / denom;
    let w_perp = [
        u_perp[0] / (g * denom),
        u_perp[1] / (g * denom),
        u_perp[2] / (g * denom),
    ];
    [
        w_par * dir[0] + w_perp[0],
        w_par * dir[1] + w_perp[1],
        w_par * dir[2] + w_perp[2],
    ]
}

pub fn relativistic_doppler(freq: f64, v: f64, angle: f64) -> f64 {
    let b = v / C;
    let g = super::lorentz::gamma_factor(v);
    freq / (g * (1.0 + b * angle.cos()))
}

pub fn transverse_doppler(freq: f64, v: f64) -> f64 {
    freq / super::lorentz::gamma_factor(v)
}

pub fn aberration(theta: f64, v: f64) -> f64 {
    let b = v / C;
    let g = super::lorentz::gamma_factor(v);
    ((theta.sin()) / (g * (theta.cos() + b))).atan()
}

pub fn headlight_effect(theta_rest: f64, v: f64) -> f64 {
    let b = v / C;
    let cos_rest = theta_rest.cos();
    let cos_lab = (cos_rest + b) / (1.0 + b * cos_rest);
    cos_lab.acos()
}

pub fn proper_acceleration_to_coordinate(proper_accel: f64, proper_time: f64) -> (f64, f64) {
    let a = proper_accel;
    let tau = proper_time;
    let t = (C / a) * (a * tau / C).sinh();
    let x = (C * C / a) * ((a * tau / C).cosh() - 1.0);
    (t, x)
}

pub fn twin_paradox_age(v: f64, coord_time: f64) -> f64 {
    coord_time / super::lorentz::gamma_factor(v)
}
