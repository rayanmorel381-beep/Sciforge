pub fn single_slit_intensity(theta: f64, a: f64, wavelength: f64) -> f64 {
    let beta = std::f64::consts::PI * a * theta.sin() / wavelength;
    if beta.abs() < 1e-15 {
        return 1.0;
    }
    (beta.sin() / beta).powi(2)
}

pub fn double_slit_intensity(theta: f64, d: f64, wavelength: f64) -> f64 {
    let delta = std::f64::consts::PI * d * theta.sin() / wavelength;
    delta.cos().powi(2)
}

pub fn diffraction_grating_maxima(d: f64, wavelength: f64, order: i32) -> f64 {
    let sin_theta = order as f64 * wavelength / d;
    if sin_theta.abs() > 1.0 {
        return f64::NAN;
    }
    sin_theta.asin()
}

pub fn resolving_power_grating(order: i32, n_slits: u32) -> f64 {
    order.abs() as f64 * n_slits as f64
}

pub fn rayleigh_criterion(wavelength: f64, aperture: f64) -> f64 {
    1.22 * wavelength / aperture
}

pub fn airy_disk_radius(wavelength: f64, f_number: f64) -> f64 {
    1.22 * wavelength * f_number
}

pub fn fraunhofer_distance(aperture: f64, wavelength: f64) -> f64 {
    2.0 * aperture * aperture / wavelength
}

pub fn grating_dispersion(order: i32, d: f64, theta: f64) -> f64 {
    order as f64 / (d * theta.cos())
}

pub fn bragg_condition(d_spacing: f64, theta: f64, wavelength: f64) -> f64 {
    2.0 * d_spacing * theta.sin() / wavelength
}

pub fn circular_aperture_first_zero(wavelength: f64, diameter: f64) -> f64 {
    1.22 * wavelength / diameter
}
