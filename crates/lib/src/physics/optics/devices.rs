pub fn sellmeier(b1: f64, c1_um2: f64, b2: f64, c2_um2: f64, b3: f64, c3_um2: f64, wavelength_um: f64) -> f64 {
    let l2 = wavelength_um * wavelength_um;
    let n2 = 1.0
        + b1 * l2 / (l2 - c1_um2)
        + b2 * l2 / (l2 - c2_um2)
        + b3 * l2 / (l2 - c3_um2);
    n2.sqrt()
}

pub fn cauchy_n(a: f64, b: f64, c: f64, wavelength_um: f64) -> f64 {
    let l2 = wavelength_um * wavelength_um;
    a + b / l2 + c / (l2 * l2)
}

pub fn abbe_number(n_d: f64, n_f: f64, n_c: f64) -> f64 {
    (n_d - 1.0) / (n_f - n_c)
}

pub fn beer_lambert(i0: f64, alpha_per_m: f64, thickness_m: f64) -> f64 {
    i0 * (-alpha_per_m * thickness_m).exp()
}

pub fn penetration_depth(alpha_per_m: f64) -> f64 {
    1.0 / alpha_per_m
}

pub fn optical_path(n: f64, geometric_path_m: f64) -> f64 {
    n * geometric_path_m
}

pub fn fresnel_reflectance_s(n1: f64, n2: f64, theta_i: f64) -> f64 {
    let cos_i = theta_i.cos();
    let sin_t2 = (n1 / n2).powi(2) * theta_i.sin().powi(2);
    if sin_t2 >= 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin_t2).sqrt();
    ((n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t)).powi(2)
}

pub fn fresnel_reflectance_p(n1: f64, n2: f64, theta_i: f64) -> f64 {
    let cos_i = theta_i.cos();
    let sin_t2 = (n1 / n2).powi(2) * theta_i.sin().powi(2);
    if sin_t2 >= 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin_t2).sqrt();
    ((n2 * cos_i - n1 * cos_t) / (n2 * cos_i + n1 * cos_t)).powi(2)
}

pub fn fresnel_transmittance_s(n1: f64, n2: f64, theta_i: f64) -> f64 {
    1.0 - fresnel_reflectance_s(n1, n2, theta_i)
}

pub fn fresnel_transmittance_p(n1: f64, n2: f64, theta_i: f64) -> f64 {
    1.0 - fresnel_reflectance_p(n1, n2, theta_i)
}

pub fn lensmaker_thin(n: f64, r1_m: f64, r2_m: f64) -> f64 {
    1.0 / ((n - 1.0) * (1.0 / r1_m - 1.0 / r2_m))
}

pub fn lensmaker_thick(n: f64, r1_m: f64, r2_m: f64, thickness_m: f64) -> f64 {
    let inv_f = (n - 1.0) * (1.0 / r1_m - 1.0 / r2_m + (n - 1.0) * thickness_m / (n * r1_m * r2_m));
    1.0 / inv_f
}

pub fn pockels_index_change(n0: f64, r_pm_per_v: f64, e_field_v_per_m: f64) -> f64 {
    -0.5 * n0.powi(3) * (r_pm_per_v * 1.0e-12) * e_field_v_per_m
}

pub fn kerr_index_change(n2_m2_per_w: f64, intensity_w_per_m2: f64) -> f64 {
    n2_m2_per_w * intensity_w_per_m2
}

pub fn fiber_numerical_aperture(n_core: f64, n_clad: f64) -> f64 {
    (n_core * n_core - n_clad * n_clad).sqrt()
}

pub fn fiber_v_number(core_radius_m: f64, na: f64, wavelength_m: f64) -> f64 {
    2.0 * std::f64::consts::PI * core_radius_m * na / wavelength_m
}

pub fn fiber_acceptance_angle(na: f64, n_medium: f64) -> f64 {
    (na / n_medium).asin()
}

pub fn n_temperature(n_ref: f64, dn_dt_per_k: f64, t_k: f64, t_ref_k: f64) -> f64 {
    n_ref + dn_dt_per_k * (t_k - t_ref_k)
}

pub fn group_index(n: f64, wavelength_m: f64, dn_dlambda_per_m: f64) -> f64 {
    n - wavelength_m * dn_dlambda_per_m
}
