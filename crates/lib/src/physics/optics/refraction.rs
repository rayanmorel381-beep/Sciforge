pub fn snell(n1: f64, theta1: f64, n2: f64) -> f64 {
    let sin_theta2 = n1 * theta1.sin() / n2;
    if sin_theta2.abs() > 1.0 {
        return std::f64::consts::FRAC_PI_2;
    }
    sin_theta2.asin()
}

pub fn critical_angle(n1: f64, n2: f64) -> f64 {
    if n2 >= n1 {
        return std::f64::consts::FRAC_PI_2;
    }
    (n2 / n1).asin()
}

pub fn brewster_angle(n1: f64, n2: f64) -> f64 {
    (n2 / n1).atan()
}

pub fn fresnel_reflectance_s(n1: f64, theta_i: f64, n2: f64, theta_t: f64) -> f64 {
    let num = n1 * theta_i.cos() - n2 * theta_t.cos();
    let den = n1 * theta_i.cos() + n2 * theta_t.cos();
    (num / den).powi(2)
}

pub fn fresnel_reflectance_p(n1: f64, theta_i: f64, n2: f64, theta_t: f64) -> f64 {
    let num = n2 * theta_i.cos() - n1 * theta_t.cos();
    let den = n2 * theta_i.cos() + n1 * theta_t.cos();
    (num / den).powi(2)
}

pub fn thin_lens_equation(focal: f64, object_dist: f64) -> f64 {
    1.0 / (1.0 / focal - 1.0 / object_dist)
}

pub fn magnification(image_dist: f64, object_dist: f64) -> f64 {
    -image_dist / object_dist
}

pub fn lensmaker_equation(n: f64, r1: f64, r2: f64) -> f64 {
    (n - 1.0) * (1.0 / r1 - 1.0 / r2)
}

pub fn numerical_aperture(n: f64, half_angle: f64) -> f64 {
    n * half_angle.sin()
}

pub fn optical_path_length(n: f64, d: f64) -> f64 {
    n * d
}

pub fn cauchy_dispersion(a: f64, b: f64, wavelength: f64) -> f64 {
    a + b / (wavelength * wavelength)
}

pub fn abbe_number(nd: f64, nf: f64, nc: f64) -> f64 {
    (nd - 1.0) / (nf - nc)
}
