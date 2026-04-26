use std::f64::consts::PI;

use super::distances::{
    angular_diameter_distance_from_z, comoving_distance_from_z, distance_modulus_from_z,
    luminosity_distance_from_z,
};
use super::expansion::{critical_density_from_h0, e_z_lcdm, hubble_time_gyr, quad};
use crate::constants::{
    C, C_KM_S, CMB_TEMPERATURE, G, MPC_IN_KM, MPC_IN_M, SEC_PER_GYR, SIGMA_SB, SOLAR_MASS,
};

pub fn comoving_volume_element(h0: f64, omega_m: f64, z: f64) -> f64 {
    let d_h = C_KM_S / h0;
    let dc = comoving_distance_from_z(h0, omega_m, z);
    d_h * dc * dc / e_z_lcdm(omega_m, z)
}

pub fn comoving_volume_total(h0: f64, omega_m: f64, z: f64) -> f64 {
    let dc = comoving_distance_from_z(h0, omega_m, z);
    4.0 / 3.0 * PI * dc * dc * dc
}

pub fn comoving_volume_shell(h0: f64, omega_m: f64, z1: f64, z2: f64) -> f64 {
    (comoving_volume_total(h0, omega_m, z2) - comoving_volume_total(h0, omega_m, z1)).abs()
}

pub fn matter_density_at_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    critical_density_from_h0(h0) * omega_m * (1.0 + z).powi(3)
}

pub fn radiation_density_at_z(h0: f64, omega_r: f64, z: f64) -> f64 {
    critical_density_from_h0(h0) * omega_r * (1.0 + z).powi(4)
}

pub fn dark_energy_density_at_z(h0: f64, omega_de: f64, w: f64, z: f64) -> f64 {
    critical_density_from_h0(h0) * omega_de * (1.0 + z).powf(3.0 * (1.0 + w))
}

pub fn omega_m_at_z(omega_m: f64, z: f64) -> f64 {
    let e2 = e_z_lcdm(omega_m, z);
    omega_m * (1.0 + z).powi(3) / (e2 * e2)
}

pub fn omega_de_at_z(omega_m: f64, z: f64) -> f64 {
    let e2 = e_z_lcdm(omega_m, z);
    (1.0 - omega_m) / (e2 * e2)
}

pub fn growth_factor_approx(omega_m: f64, z: f64) -> f64 {
    let om_z = omega_m_at_z(omega_m, z);
    let ol_z = omega_de_at_z(omega_m, z);
    let d_z = 2.5 * om_z / (om_z.powf(4.0 / 7.0) - ol_z + (1.0 + om_z / 2.0) * (1.0 + ol_z / 70.0));
    let om_0 = omega_m;
    let ol_0 = 1.0 - omega_m;
    let d_0 = 2.5 * om_0 / (om_0.powf(4.0 / 7.0) - ol_0 + (1.0 + om_0 / 2.0) * (1.0 + ol_0 / 70.0));
    (d_z / (1.0 + z)) / d_0
}

pub fn growth_rate(omega_m: f64, z: f64) -> f64 {
    omega_m_at_z(omega_m, z).powf(0.55)
}

pub fn sigma8_at_z(sigma8: f64, omega_m: f64, z: f64) -> f64 {
    sigma8 * growth_factor_approx(omega_m, z)
}

pub fn cmb_temperature_at_z(z: f64) -> f64 {
    CMB_TEMPERATURE * (1.0 + z)
}

pub fn photon_number_density(z: f64) -> f64 {
    4.107e8 * (1.0 + z).powi(3)
}

pub fn cmb_energy_density(z: f64) -> f64 {
    let t = CMB_TEMPERATURE * (1.0 + z);
    4.0 * SIGMA_SB / C * t.powi(4)
}

pub fn omega_r_from_tcmb(t_cmb: f64, h0: f64) -> f64 {
    let rho_r = 4.0 * SIGMA_SB * t_cmb.powi(4) / (C * C * C);

    let rho_c = critical_density_from_h0(h0);
    rho_r / rho_c
}

pub fn einstein_radius(h0: f64, omega_m: f64, mass: f64, z_l: f64, z_s: f64) -> f64 {
    let d_l = angular_diameter_distance_from_z(h0, omega_m, z_l);
    let d_s = angular_diameter_distance_from_z(h0, omega_m, z_s);

    let dc_l = comoving_distance_from_z(h0, omega_m, z_l);
    let dc_s = comoving_distance_from_z(h0, omega_m, z_s);
    let d_ls = (dc_s - dc_l) / (1.0 + z_s);
    let d_l_m = d_l * MPC_IN_M;
    let d_s_m = d_s * MPC_IN_M;
    let d_ls_m = d_ls * MPC_IN_M;
    (4.0 * G * mass / (C * C) * d_ls_m / (d_l_m * d_s_m)).sqrt()
}

pub fn critical_surface_density(h0: f64, omega_m: f64, z_l: f64, z_s: f64) -> f64 {
    let d_l = angular_diameter_distance_from_z(h0, omega_m, z_l) * MPC_IN_M;
    let d_s = angular_diameter_distance_from_z(h0, omega_m, z_s) * MPC_IN_M;
    let dc_l = comoving_distance_from_z(h0, omega_m, z_l);
    let dc_s = comoving_distance_from_z(h0, omega_m, z_s);
    let d_ls = (dc_s - dc_l) / (1.0 + z_s) * MPC_IN_M;
    C * C / (4.0 * PI * G) * d_s / (d_l * d_ls)
}

pub fn sn1a_apparent_magnitude(h0: f64, omega_m: f64, z: f64, abs_mag: f64) -> f64 {
    abs_mag + distance_modulus_from_z(h0, omega_m, z)
}

pub fn sn1a_chi2_single(m_obs: f64, m_model: f64, sigma: f64) -> f64 {
    let dm = m_obs - m_model;
    dm * dm / (sigma * sigma)
}

pub fn sn1a_chi2_total(h0: f64, omega_m: f64, abs_mag: f64, data: &[(f64, f64, f64)]) -> f64 {
    data.iter()
        .map(|&(z, m_obs, sigma)| {
            let m_model = sn1a_apparent_magnitude(h0, omega_m, z, abs_mag);
            sn1a_chi2_single(m_obs, m_model, sigma)
        })
        .sum()
}

pub fn dv_bao(h0: f64, omega_m: f64, z: f64) -> f64 {
    let dm = comoving_distance_from_z(h0, omega_m, z);
    let hz = h0 * e_z_lcdm(omega_m, z);
    (dm * dm * C_KM_S * z / hz).powf(1.0 / 3.0)
}

pub fn hubble_distance_at_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    C_KM_S / (h0 * e_z_lcdm(omega_m, z))
}

pub fn rd_over_dv(r_d: f64, h0: f64, omega_m: f64, z: f64) -> f64 {
    r_d / dv_bao(h0, omega_m, z)
}

pub fn virial_radius(h0: f64, omega_m: f64, mass_solar: f64, z: f64) -> f64 {
    let rho_c = critical_density_from_h0(h0) * e_z_lcdm(omega_m, z).powi(2);
    let m_kg = mass_solar * SOLAR_MASS;
    let r_m = (3.0 * m_kg / (4.0 * PI * 200.0 * rho_c)).powf(1.0 / 3.0);
    r_m / MPC_IN_M
}

pub fn nfw_concentration_duffy08(mass_solar: f64, z: f64) -> f64 {
    let a = 5.71;
    let b = -0.084;
    let c = -0.47;
    let m_pivot = 2.0e12;
    a * (mass_solar / m_pivot).powf(b) * (1.0 + z).powf(c)
}

pub fn virial_velocity(h0: f64, omega_m: f64, mass_solar: f64, z: f64) -> f64 {
    let r_vir = virial_radius(h0, omega_m, mass_solar, z) * MPC_IN_M;
    let m_kg = mass_solar * SOLAR_MASS;
    (G * m_kg / r_vir).sqrt() / 1e3
}

pub fn photon_density_today() -> f64 {
    4.107e8
}

pub fn baryon_to_photon_ratio(omega_b: f64, h0: f64) -> f64 {
    let h = h0 / 100.0;
    2.74e-8 * omega_b * h * h
}

pub fn neutrino_temperature() -> f64 {
    CMB_TEMPERATURE * (4.0_f64 / 11.0).powf(1.0 / 3.0)
}

pub fn n_eff_standard() -> f64 {
    3.044
}

pub fn omega_r_with_neutrinos(omega_gamma: f64, n_eff: f64) -> f64 {
    omega_gamma * (1.0 + n_eff * 7.0 / 8.0 * (4.0_f64 / 11.0).powf(4.0 / 3.0))
}

pub fn jeans_length(sound_speed: f64, density: f64) -> f64 {
    sound_speed * (PI / (G * density)).sqrt()
}

pub fn jeans_mass(sound_speed: f64, density: f64) -> f64 {
    let lj = jeans_length(sound_speed, density);
    4.0 / 3.0 * PI * density * (lj / 2.0).powi(3)
}

pub fn redshift_from_temperature(t: f64) -> f64 {
    t / CMB_TEMPERATURE - 1.0
}

pub fn conformal_time(h0: f64, omega_m: f64, z: f64) -> f64 {
    hubble_time_gyr(h0) * quad(0.0, z, 1000, |zi| 1.0 / e_z_lcdm(omega_m, zi))
}

pub fn conformal_distance(h0: f64, omega_m: f64, z: f64) -> f64 {
    comoving_distance_from_z(h0, omega_m, z)
}

pub fn luminosity_distance_km(h0: f64, omega_m: f64, z: f64) -> f64 {
    luminosity_distance_from_z(h0, omega_m, z) * MPC_IN_KM
}

pub fn hubble_parameter_at_z(h0: f64, omega_m: f64, z: f64) -> f64 {
    h0 * e_z_lcdm(omega_m, z) / MPC_IN_KM * SEC_PER_GYR
}
