use crate::constants::{HC_EV_NM, K_B, KELVIN_TO_KEV};

pub fn fermi_dirac(energy: f64, fermi_level: f64, t: f64) -> f64 {
    let x = (energy - fermi_level) / (K_B * t).max(1e-30);
    if x > 500.0 {
        return 0.0;
    }
    1.0 / (1.0 + x.exp())
}

pub fn band_gap_from_absorption(wavelength_edge_nm: f64) -> f64 {
    HC_EV_NM / wavelength_edge_nm.max(1e-30)
}

pub fn intrinsic_carrier_concentration(nc: f64, nv: f64, eg: f64, t: f64) -> f64 {
    (nc * nv).sqrt() * (-eg / (2.0 * KELVIN_TO_KEV * t)).exp()
}

pub fn conductivity_semiconductor(n: f64, mu_e: f64, p: f64, mu_h: f64) -> f64 {
    crate::constants::E_CHARGE * (n * mu_e + p * mu_h)
}

pub fn hall_coefficient(n: f64, p: f64) -> f64 {
    (p - n) / (crate::constants::E_CHARGE * (p + n).powi(2)).max(1e-30)
}

pub fn doping_carrier_concentration(nd: f64, ni: f64) -> (f64, f64) {
    let n = nd;
    let p = ni * ni / nd.max(1e-30);
    (n, p)
}

pub fn resistivity_from_conductivity(sigma: f64) -> f64 {
    1.0 / sigma.max(1e-30)
}

pub fn seebeck_coefficient(delta_v: f64, delta_t: f64) -> f64 {
    delta_v / delta_t.max(1e-30)
}

pub fn ionic_conductivity_arrhenius(sigma0: f64, ea: f64, t: f64) -> f64 {
    sigma0 / t * (-ea / (KELVIN_TO_KEV * t)).exp()
}
