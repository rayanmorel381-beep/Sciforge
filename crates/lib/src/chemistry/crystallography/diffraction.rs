pub fn bragg_law(d: f64, theta: f64, n: u32) -> f64 {
    2.0 * d * theta.sin() / n.max(1) as f64
}

pub fn bragg_angle(wavelength: f64, d: f64, n: u32) -> f64 {
    (n as f64 * wavelength / (2.0 * d).max(1e-30)).asin()
}

pub fn interplanar_spacing_cubic(a: f64, h: i32, k: i32, l: i32) -> f64 {
    a / ((h * h + k * k + l * l) as f64).sqrt().max(1e-30)
}

pub fn structure_factor_bcc(h: i32, k: i32, l: i32, f_atom: f64) -> f64 {
    if (h + k + l) % 2 == 0 {
        2.0 * f_atom
    } else {
        0.0
    }
}

pub fn structure_factor_fcc(h: i32, k: i32, l: i32, f_atom: f64) -> f64 {
    let all_even = h % 2 == 0 && k % 2 == 0 && l % 2 == 0;
    let all_odd = h % 2 != 0 && k % 2 != 0 && l % 2 != 0;
    if all_even || all_odd {
        4.0 * f_atom
    } else {
        0.0
    }
}

pub fn scherrer_crystal_size(k: f64, wavelength: f64, fwhm: f64, theta: f64) -> f64 {
    k * wavelength / (fwhm * theta.cos()).max(1e-30)
}
