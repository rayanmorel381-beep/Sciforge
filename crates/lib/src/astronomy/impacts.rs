use crate::constants::{
    CRATER_DENSITY_EXPONENT, CRATER_GRAVITY_EXPONENT, CRATER_PROJECTILE_EXPONENT,
    CRATER_SCALING_COEFF, CRATER_VELOCITY_EXPONENT, FIREBALL_ENERGY_EXPONENT,
    FIREBALL_RADIUS_COEFF,
};

pub fn crater_diameter(rho_i: f64, d_p: f64, v: f64, g: f64, rho_t: f64) -> f64 {
    let density_ratio = rho_i / rho_t;
    CRATER_SCALING_COEFF
        * density_ratio.powf(CRATER_DENSITY_EXPONENT)
        * d_p.powf(CRATER_PROJECTILE_EXPONENT)
        * v.powf(CRATER_VELOCITY_EXPONENT)
        * g.powf(CRATER_GRAVITY_EXPONENT)
}

pub fn fireball_radius(e_kt: f64) -> f64 {
    FIREBALL_RADIUS_COEFF * e_kt.powf(FIREBALL_ENERGY_EXPONENT)
}

pub fn ejecta_volume(d: f64, depth: f64) -> f64 {
    std::f64::consts::PI / 6.0 * d.powi(2) * depth
}

pub fn impact_velocity(v_inf: f64, v_esc: f64) -> f64 {
    (v_inf.powi(2) + v_esc.powi(2)).sqrt()
}

pub fn ejecta_escape_fraction(v_esc: f64, v_ejecta: f64) -> f64 {
    if v_ejecta <= v_esc {
        return 0.0;
    }
    (1.0 - (v_esc / v_ejecta).powi(2)).max(0.0)
}
