pub fn thin_cylinder_hoop_stress(p_pa: f64, r_m: f64, t_m: f64) -> f64 {
    p_pa * r_m / t_m
}

pub fn thin_cylinder_axial_stress(p_pa: f64, r_m: f64, t_m: f64) -> f64 {
    p_pa * r_m / (2.0 * t_m)
}

pub fn thin_sphere_stress(p_pa: f64, r_m: f64, t_m: f64) -> f64 {
    p_pa * r_m / (2.0 * t_m)
}

pub fn lame_radial_stress(
    p_inner_pa: f64,
    p_outer_pa: f64,
    r_inner_m: f64,
    r_outer_m: f64,
    r_m: f64,
) -> f64 {
    let a = r_inner_m;
    let b = r_outer_m;
    let pi = p_inner_pa;
    let po = p_outer_pa;
    (pi * a * a - po * b * b) / (b * b - a * a)
        - (pi - po) * a * a * b * b / ((b * b - a * a) * r_m * r_m)
}

pub fn lame_hoop_stress(
    p_inner_pa: f64,
    p_outer_pa: f64,
    r_inner_m: f64,
    r_outer_m: f64,
    r_m: f64,
) -> f64 {
    let a = r_inner_m;
    let b = r_outer_m;
    let pi = p_inner_pa;
    let po = p_outer_pa;
    (pi * a * a - po * b * b) / (b * b - a * a)
        + (pi - po) * a * a * b * b / ((b * b - a * a) * r_m * r_m)
}

pub fn asme_thickness_internal_pressure(
    p_pa: f64,
    r_m: f64,
    s_allow_pa: f64,
    weld_efficiency: f64,
) -> f64 {
    p_pa * r_m / (s_allow_pa * weld_efficiency - 0.6 * p_pa)
}

pub fn asme_sphere_thickness(
    p_pa: f64,
    r_m: f64,
    s_allow_pa: f64,
    weld_efficiency: f64,
) -> f64 {
    p_pa * r_m / (2.0 * s_allow_pa * weld_efficiency - 0.2 * p_pa)
}

pub fn burst_pressure_thin_cylinder(sigma_u_pa: f64, t_m: f64, r_m: f64) -> f64 {
    sigma_u_pa * t_m / r_m
}

pub fn autofrettage_residual_stress(
    p_auto_pa: f64,
    r_inner_m: f64,
    r_outer_m: f64,
    r_plastic_m: f64,
    sigma_y_pa: f64,
) -> f64 {
    let a = r_inner_m;
    let b = r_outer_m;
    let c = r_plastic_m;
    let factor = sigma_y_pa
        * ((c / a).ln() + (b * b - c * c) / (2.0 * b * b));
    p_auto_pa - factor
}

pub fn slenderness_ratio_pressure_vessel(length_m: f64, radius_m: f64) -> f64 {
    length_m / radius_m
}
