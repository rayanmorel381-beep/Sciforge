use std::f64::consts::PI;

pub fn euler_critical_stress(e_pa: f64, slenderness: f64) -> f64 {
    PI * PI * e_pa / (slenderness * slenderness)
}

pub fn johnson_short_column(sigma_y_pa: f64, e_pa: f64, slenderness: f64) -> f64 {
    let cc = (2.0 * PI * PI * e_pa / sigma_y_pa).sqrt();
    if slenderness >= cc {
        PI * PI * e_pa / (slenderness * slenderness)
    } else {
        sigma_y_pa * (1.0 - sigma_y_pa * (slenderness * slenderness) / (4.0 * PI * PI * e_pa))
    }
}

pub fn effective_length(length_m: f64, end_condition: &str) -> f64 {
    let k = match end_condition {
        "fixed-fixed" => 0.5,
        "fixed-pinned" => 0.7,
        "pinned-pinned" => 1.0,
        "fixed-free" => 2.0,
        _ => 1.0,
    };
    k * length_m
}

pub fn plate_buckling_critical_stress(
    e_pa: f64,
    nu: f64,
    thickness_m: f64,
    width_m: f64,
    k_factor: f64,
) -> f64 {
    k_factor * PI * PI * e_pa / (12.0 * (1.0 - nu * nu)) * (thickness_m / width_m).powi(2)
}

pub fn cylindrical_shell_buckling(
    e_pa: f64,
    thickness_m: f64,
    radius_m: f64,
    nu: f64,
) -> f64 {
    e_pa * thickness_m / (radius_m * (3.0 * (1.0 - nu * nu)).sqrt())
}
