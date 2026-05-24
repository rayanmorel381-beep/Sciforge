use crate::constants::{
    FARQUHAR_WJ_CI_COEFF, FARQUHAR_WJ_GAMMA_COEFF, R_GAS, REFERENCE_TEMP_KELVIN,
};

pub fn farquhar_c3(
    vcmax: f64,
    ci: f64,
    gamma_star: f64,
    kc: f64,
    ko: f64,
    o: f64,
    j: f64,
    rd: f64,
) -> f64 {
    let wc = vcmax * (ci - gamma_star) / (ci + kc * (1.0 + o / ko));
    let wj =
        j * (ci - gamma_star) / (FARQUHAR_WJ_CI_COEFF * ci + FARQUHAR_WJ_GAMMA_COEFF * gamma_star);
    wc.min(wj) - rd
}

pub fn light_response_curve(phi: f64, ppfd: f64, amax: f64, theta: f64, rd: f64) -> f64 {
    let a = theta;
    let b = -(phi * ppfd + amax);
    let c = phi * ppfd * amax;
    let disc = (b * b - 4.0 * a * c).max(0.0).sqrt();
    (-b - disc) / (2.0 * a) - rd
}

pub fn transpiration_rate(stomatal_conductance: f64, vpd: f64) -> f64 {
    stomatal_conductance * vpd
}

pub fn ball_berry_conductance(a_n: f64, cs: f64, rh: f64, g0: f64, g1: f64) -> f64 {
    g0 + g1 * a_n * rh / cs
}

pub fn water_use_efficiency(a_n: f64, transpiration: f64) -> f64 {
    a_n / transpiration
}

pub fn penman_monteith(
    net_radiation: f64,
    soil_heat: f64,
    vpd: f64,
    ga: f64,
    gs: f64,
    delta: f64,
    gamma: f64,
    rho: f64,
    cp: f64,
) -> f64 {
    let numerator = delta * (net_radiation - soil_heat) + rho * cp * vpd * ga;
    let denominator = delta + gamma * (1.0 + ga / gs);
    numerator / denominator
}

pub fn rubisco_specificity(vcmax: f64, kc: f64, vomax: f64, ko: f64) -> f64 {
    (vcmax / kc) / (vomax / ko)
}

pub fn photorespiration_rate(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) -> f64 {
    vomax * o / (ko * (1.0 + ci / kc) + o)
}

pub fn electron_transport_rate(
    ppfd: f64,
    absorptance: f64,
    fraction_psii: f64,
    phi_psii: f64,
) -> f64 {
    ppfd * absorptance * fraction_psii * phi_psii
}

pub fn stomatal_optimization(vpd: f64, ca: f64, lambda_wue: f64, g1: f64) -> f64 {
    g1 / ((vpd / lambda_wue).sqrt()) / ca
}

pub fn c4_photosynthesis(
    vpmax: f64,
    ci: f64,
    kp: f64,
    vcmax: f64,
    ko: f64,
    kc: f64,
    o: f64,
    rd: f64,
) -> f64 {
    let vp = vpmax * ci / (ci + kp);
    let vc = vcmax * ci / (ci + kc * (1.0 + o / ko));
    vp.min(vc) - rd
}

pub fn cam_malic_acid_storage(
    co2_fixed_night: f64,
    vacuole_capacity: f64,
    current_malate: f64,
) -> f64 {
    let space = (vacuole_capacity - current_malate).max(0.0);
    co2_fixed_night.min(space)
}

pub fn cam_daytime_decarboxylation(malate: f64, decarboxylation_rate: f64) -> f64 {
    decarboxylation_rate * malate
}

pub fn chlorophyll_fluorescence_fv_fm(f0: f64, fm: f64) -> f64 {
    (fm - f0) / fm
}

pub fn non_photochemical_quenching(fm: f64, fm_prime: f64) -> f64 {
    (fm - fm_prime) / fm_prime
}

pub fn photochemical_quenching(fs: f64, f0_prime: f64, fm_prime: f64) -> f64 {
    (fm_prime - fs) / (fm_prime - f0_prime)
}

pub fn quantum_yield_psii(phi_psii: f64, ppfd: f64) -> f64 {
    phi_psii * ppfd
}

pub fn co2_compensation_point(
    gamma_star: f64,
    rd: f64,
    vcmax: f64,
    kc: f64,
    ko: f64,
    o: f64,
) -> f64 {
    gamma_star + rd * (kc * (1.0 + o / ko)) / (vcmax - rd)
}

pub fn mesophyll_conductance(a_n: f64, ci: f64, cc: f64) -> f64 {
    a_n / (ci - cc)
}

pub fn light_use_efficiency(gpp: f64, apar: f64) -> f64 {
    gpp / apar
}

pub fn vcmax_temperature_response(vcmax25: f64, ha: f64, temp_k: f64) -> f64 {
    vcmax25
        * (ha * (temp_k - REFERENCE_TEMP_KELVIN) / (REFERENCE_TEMP_KELVIN * R_GAS * temp_k)).exp()
}

pub fn jmax_temperature_peaked(jmax25: f64, ha: f64, hd: f64, ds: f64, temp_k: f64) -> f64 {
    let num =
        (ha * (temp_k - REFERENCE_TEMP_KELVIN) / (REFERENCE_TEMP_KELVIN * R_GAS * temp_k)).exp();
    let denom_ref =
        1.0 + ((ds * REFERENCE_TEMP_KELVIN - hd) / (R_GAS * REFERENCE_TEMP_KELVIN)).exp();
    let denom_t = 1.0 + ((ds * temp_k - hd) / (R_GAS * temp_k)).exp();
    jmax25 * num * denom_ref / denom_t
}
