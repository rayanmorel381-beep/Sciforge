pub fn ozone_formation_rate(k: f64, no2: f64, hv: f64) -> f64 {
    k * no2 * hv
}

pub fn ozone_destruction_rate(k: f64, o3: f64, no: f64) -> f64 {
    k * o3 * no
}

pub fn photochemical_smog_potential(voc: f64, nox: f64) -> f64 {
    voc / nox.max(1e-30)
}

pub fn atmospheric_lifetime(concentration: f64, removal_rate: f64) -> f64 {
    concentration / removal_rate.max(1e-30)
}

pub fn global_warming_potential(
    rf_gas: f64,
    lifetime_gas: f64,
    rf_co2: f64,
    lifetime_co2: f64,
) -> f64 {
    rf_gas * lifetime_gas / (rf_co2 * lifetime_co2).max(1e-30)
}

pub fn henry_law_solubility(kh: f64, partial_pressure: f64) -> f64 {
    kh * partial_pressure
}

pub fn ozone_depletion_potential(cl_atoms: f64, lifetime: f64, mw: f64) -> f64 {
    cl_atoms * lifetime * 17.0 / mw.max(1e-30)
}

pub fn photolysis_rate_constant(quantum_yield: f64, absorption: f64, actinic_flux: f64) -> f64 {
    quantum_yield * absorption * actinic_flux
}
