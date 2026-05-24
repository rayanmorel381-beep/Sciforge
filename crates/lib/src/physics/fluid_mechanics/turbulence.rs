pub fn turbulent_kinetic_energy(u_prime: f64, v_prime: f64, w_prime: f64) -> f64 {
    0.5 * (u_prime * u_prime + v_prime * v_prime + w_prime * w_prime)
}

pub fn kolmogorov_length_scale(nu: f64, epsilon: f64) -> f64 {
    (nu.powi(3) / epsilon).powf(0.25)
}

pub fn kolmogorov_time_scale(nu: f64, epsilon: f64) -> f64 {
    (nu / epsilon).sqrt()
}

pub fn kolmogorov_velocity_scale(nu: f64, epsilon: f64) -> f64 {
    (nu * epsilon).powf(0.25)
}

pub fn taylor_microscale(u_rms: f64, epsilon: f64, nu: f64) -> f64 {
    u_rms * (15.0 * nu / epsilon).sqrt()
}

pub fn integral_length_scale(tke: f64, epsilon: f64) -> f64 {
    tke.powf(1.5) / epsilon.max(1e-30)
}

pub fn friction_velocity(tau_wall: f64, rho: f64) -> f64 {
    (tau_wall / rho).sqrt()
}

pub fn law_of_wall(u_tau: f64, y: f64, nu: f64) -> f64 {
    let y_plus = u_tau * y / nu;
    if y_plus < 5.0 {
        u_tau * y_plus
    } else if y_plus < 30.0 {
        u_tau * (5.0 * y_plus.ln() - 3.05)
    } else {
        u_tau * (2.5 * y_plus.ln() + 5.5)
    }
}

pub fn mixing_length(kappa: f64, y: f64) -> f64 {
    kappa * y
}

pub fn eddy_viscosity(mixing_length: f64, du_dy: f64) -> f64 {
    mixing_length * mixing_length * du_dy.abs()
}

pub fn turbulence_intensity(u_rms: f64, u_mean: f64) -> f64 {
    u_rms / u_mean.max(1e-30)
}

pub fn energy_spectrum_kolmogorov(c_k: f64, epsilon: f64, k: f64) -> f64 {
    c_k * epsilon.powf(2.0 / 3.0) * k.powf(-5.0 / 3.0)
}
