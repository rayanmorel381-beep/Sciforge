use crate::constants::{DEGREE_TO_RAD, NUTATION_AMPLITUDE_ARCSEC, VERNAL_EQUINOX_DOY};

pub fn surface_velocity_at_latitude(omega: f64, r: f64, phi: f64) -> f64 {
    omega * r * phi.cos()
}

pub fn centripetal_acceleration(omega: f64, r: f64, phi: f64) -> f64 {
    omega.powi(2) * r * phi.cos()
}

pub fn coriolis_parameter(omega: f64, phi: f64) -> f64 {
    2.0 * omega * phi.sin()
}

pub fn moment_of_inertia(c_factor: f64, m: f64, r: f64) -> f64 {
    c_factor * m * r.powi(2)
}

pub fn rotational_kinetic_energy(inertia: f64, omega: f64) -> f64 {
    0.5 * inertia * omega.powi(2)
}

pub fn nutation_obliquity_rad(omega_node: f64) -> f64 {
    let arcsec_to_rad = DEGREE_TO_RAD / 3600.0;
    NUTATION_AMPLITUDE_ARCSEC * arcsec_to_rad * omega_node.cos()
}

pub fn day_length_variation(doy: f64, latitude: f64, tilt: f64) -> f64 {
    let declination =
        tilt * ((2.0 * std::f64::consts::PI / 365.0) * (doy - VERNAL_EQUINOX_DOY)).sin();
    let cos_h0 = -(latitude.tan() * declination.tan());
    if cos_h0 <= -1.0 {
        return 24.0;
    }
    if cos_h0 >= 1.0 {
        return 0.0;
    }
    24.0 * cos_h0.acos() / std::f64::consts::PI
}
