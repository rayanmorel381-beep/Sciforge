pub fn farquhar_model(
    vcmax: f64,
    jmax: f64,
    ci: f64,
    gamma_star: f64,
    kc: f64,
    ko: f64,
    o: f64,
    rd: f64,
    par: f64,
) -> f64 {
    let wc = vcmax * (ci - gamma_star) / (ci + kc * (1.0 + o / ko));
    let j = electron_transport_rate(jmax, par);
    let wj = j * (ci - gamma_star) / (4.0 * ci + 8.0 * gamma_star);
    let a_gross = wc.min(wj);
    (a_gross - rd).max(0.0)
}

pub fn electron_transport_rate(jmax: f64, par: f64) -> f64 {
    let alpha = 0.85;
    let theta = 0.7;
    let a = theta;
    let b = -(alpha * par + jmax);
    let c = alpha * par * jmax;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return 0.0;
    }
    (-b - disc.sqrt()) / (2.0 * a)
}

pub fn light_response_curve(amax: f64, phi: f64, par: f64, rd: f64) -> f64 {
    amax * (1.0 - (-phi * par / amax.max(1e-30)).exp()) - rd
}

pub fn light_compensation_point(amax: f64, phi: f64, rd: f64) -> f64 {
    if phi < 1e-30 {
        return 0.0;
    }
    -(amax / phi) * (1.0 - rd / amax.max(1e-30)).ln()
}

pub fn water_use_efficiency(assimilation: f64, transpiration: f64) -> f64 {
    assimilation / transpiration.max(1e-30)
}

pub fn rubisco_specificity(vcmax: f64, kc: f64, vomax: f64, ko: f64) -> f64 {
    (vcmax * ko) / (vomax * kc)
}

pub fn photorespiration_rate(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) -> f64 {
    vomax * o / (ko * (1.0 + ci / kc) + o)
}

pub fn quantum_yield(assimilation_rate: f64, photon_flux: f64) -> f64 {
    assimilation_rate / photon_flux.max(1e-30)
}

pub fn co2_compensation_point_photo(
    gamma_star: f64,
    rd: f64,
    vcmax: f64,
    kc: f64,
    ko: f64,
    o: f64,
) -> f64 {
    gamma_star + rd * (kc * (1.0 + o / ko)) / vcmax
}

pub fn stomatal_conductance_ball_berry(
    assimilation: f64,
    rh: f64,
    cs: f64,
    g0: f64,
    g1: f64,
) -> f64 {
    g0 + g1 * assimilation * rh / cs.max(1e-30)
}

pub fn mesophyll_conductance_photo(assimilation: f64, ci: f64, cc: f64) -> f64 {
    assimilation / (ci - cc).max(1e-30)
}

pub fn triose_phosphate_utilization(tpu: f64, ci: f64, gamma_star: f64) -> f64 {
    3.0 * tpu * (ci - gamma_star) / (ci - gamma_star + 3.0 * gamma_star)
}

pub fn light_inhibition_photoinhibition(fv_fm_initial: f64, light_excess: f64, ki: f64) -> f64 {
    fv_fm_initial * (-light_excess / ki).exp()
}

pub fn canopy_photosynthesis_sun_shade(lai: f64, k_ext: f64, a_sun: f64, a_shade: f64) -> f64 {
    let f_sun = (1.0 - (-k_ext * lai).exp()) / k_ext;
    let f_shade = lai - f_sun;
    a_sun * f_sun + a_shade * f_shade
}

pub fn carbon_concentrating_mechanism_benefit(ci_c3: f64, ci_c4: f64, vcmax: f64, kc: f64) -> f64 {
    let a_c3 = vcmax * ci_c3 / (kc + ci_c3);
    let a_c4 = vcmax * ci_c4 / (kc + ci_c4);
    a_c4 - a_c3
}
