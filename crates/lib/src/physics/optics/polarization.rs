pub fn malus_law(i0: f64, theta: f64) -> f64 {
    i0 * theta.cos().powi(2)
}

pub fn brewster_reflectance(n1: f64, n2: f64) -> f64 {
    ((n1 - n2) / (n1 + n2)).powi(2)
}

pub fn stokes_parameters(ex: f64, ey: f64, delta: f64) -> [f64; 4] {
    let s0 = ex * ex + ey * ey;
    let s1 = ex * ex - ey * ey;
    let s2 = 2.0 * ex * ey * delta.cos();
    let s3 = 2.0 * ex * ey * delta.sin();
    [s0, s1, s2, s3]
}

pub fn degree_of_polarization(s: &[f64; 4]) -> f64 {
    (s[1] * s[1] + s[2] * s[2] + s[3] * s[3]).sqrt() / s[0].max(1e-30)
}

pub fn jones_rotation(theta: f64) -> [[f64; 2]; 2] {
    let c = theta.cos();
    let s = theta.sin();
    [[c, -s], [s, c]]
}

pub fn quarter_wave_plate_phase(wavelength: f64, n_fast: f64, n_slow: f64) -> f64 {
    wavelength / (4.0 * (n_slow - n_fast).abs())
}

pub fn specific_rotation(observed: f64, l: f64, c: f64) -> f64 {
    observed / (l * c)
}

pub fn ellipticity(major: f64, minor: f64) -> f64 {
    (minor / major).atan()
}

pub fn circular_dichroism(a_left: f64, a_right: f64) -> f64 {
    a_left - a_right
}

pub fn birefringence(n_extraordinary: f64, n_ordinary: f64) -> f64 {
    n_extraordinary - n_ordinary
}

pub fn retardance(birefringence: f64, thickness: f64, wavelength: f64) -> f64 {
    2.0 * std::f64::consts::PI * birefringence * thickness / wavelength
}
