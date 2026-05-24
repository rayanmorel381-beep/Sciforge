pub fn blasius_thickness(x: f64, re_x: f64) -> f64 {
    5.0 * x / re_x.sqrt()
}

pub fn displacement_thickness(x: f64, re_x: f64) -> f64 {
    1.721 * x / re_x.sqrt()
}

pub fn momentum_thickness(x: f64, re_x: f64) -> f64 {
    0.664 * x / re_x.sqrt()
}

pub fn shape_factor(displacement: f64, momentum: f64) -> f64 {
    displacement / momentum.max(1e-30)
}

pub fn skin_friction_laminar(re_x: f64) -> f64 {
    0.664 / re_x.sqrt()
}

pub fn skin_friction_turbulent(re_x: f64) -> f64 {
    0.027 / re_x.powf(1.0 / 7.0)
}

pub fn turbulent_bl_thickness(x: f64, re_x: f64) -> f64 {
    0.37 * x / re_x.powf(0.2)
}

pub fn separation_criterion(dp_dx: f64) -> bool {
    dp_dx > 0.0
}

pub fn falkner_skan_beta(m: f64) -> f64 {
    2.0 * m / (m + 1.0)
}

pub fn thermal_bl_thickness(delta: f64, pr: f64) -> f64 {
    delta / pr.powf(1.0 / 3.0)
}

pub fn nusselt_flat_plate_laminar(re: f64, pr: f64) -> f64 {
    0.332 * re.sqrt() * pr.powf(1.0 / 3.0)
}

pub fn stanton_number(nu: f64, re: f64, pr: f64) -> f64 {
    nu / (re * pr)
}
