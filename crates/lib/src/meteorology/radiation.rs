use crate::constants::{C, H, K_B, RADIATIVE_FORCING_CO2_COEFF, SIGMA_SB, SOLAR_CONSTANT};

pub fn stefan_boltzmann_flux(t: f64) -> f64 {
    SIGMA_SB * t.powi(4)
}

pub fn solar_constant() -> f64 {
    SOLAR_CONSTANT
}

pub fn albedo_reflected(solar_flux: f64, albedo: f64) -> f64 {
    solar_flux * albedo
}

pub fn effective_temperature(solar_flux: f64, albedo: f64) -> f64 {
    (solar_flux * (1.0 - albedo) / (4.0 * SIGMA_SB)).powf(0.25)
}

pub fn greenhouse_effect(t_surface: f64, t_effective: f64) -> f64 {
    t_surface - t_effective
}

pub fn optical_depth(absorption_coeff: f64, path_length: f64) -> f64 {
    absorption_coeff * path_length
}

pub fn beer_lambert(i0: f64, tau: f64) -> f64 {
    i0 * (-tau).exp()
}

pub fn planck_function(wavelength: f64, t: f64) -> f64 {
    let c1 = 2.0 * std::f64::consts::PI * H * C * C;
    let c2 = H * C / K_B;
    c1 / (wavelength.powi(5) * ((c2 / (wavelength * t)).exp() - 1.0))
}

pub fn solar_zenith_angle(lat: f64, declination: f64, hour_angle: f64) -> f64 {
    (lat.sin() * declination.sin() + lat.cos() * declination.cos() * hour_angle.cos()).acos()
}

pub fn radiative_forcing_co2(c: f64, c0: f64) -> f64 {
    RADIATIVE_FORCING_CO2_COEFF * (c / c0).ln()
}

pub fn climate_sensitivity(delta_t: f64, delta_f: f64) -> f64 {
    delta_t / delta_f
}

pub fn outgoing_longwave_radiation(t: f64, emissivity: f64) -> f64 {
    emissivity * SIGMA_SB * t.powi(4)
}
