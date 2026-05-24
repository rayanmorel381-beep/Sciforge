pub fn reynolds_pressure_1d(
    eta_pa_s: f64,
    velocity_m_per_s: f64,
    film_thickness_m: f64,
    film_thickness_gradient: f64,
    length_m: f64,
) -> f64 {
    6.0 * eta_pa_s * velocity_m_per_s * length_m * film_thickness_gradient
        / film_thickness_m.powi(3)
}

pub fn sommerfeld_number(
    radius_m: f64,
    clearance_m: f64,
    angular_velocity_rad_per_s: f64,
    eta_pa_s: f64,
    load_per_area_pa: f64,
) -> f64 {
    let ratio = radius_m / clearance_m;
    ratio * ratio * eta_pa_s * angular_velocity_rad_per_s / load_per_area_pa
}

pub fn petroff_friction_torque(
    eta_pa_s: f64,
    angular_velocity_rad_per_s: f64,
    radius_m: f64,
    length_m: f64,
    clearance_m: f64,
) -> f64 {
    use std::f64::consts::PI;
    4.0 * PI * PI * eta_pa_s * angular_velocity_rad_per_s
        * radius_m.powi(3)
        * length_m
        / clearance_m
}

pub fn slider_bearing_load(
    eta_pa_s: f64,
    velocity_m_per_s: f64,
    length_m: f64,
    width_m: f64,
    h_inlet_m: f64,
    h_outlet_m: f64,
) -> f64 {
    let k = h_inlet_m / h_outlet_m;
    let factor = (k - 1.0).ln() - 2.0 * (k - 1.0) / (k + 1.0);
    6.0 * eta_pa_s * velocity_m_per_s * length_m * length_m * width_m
        / ((k - 1.0).powi(2) * h_outlet_m * h_outlet_m)
        * factor
}
