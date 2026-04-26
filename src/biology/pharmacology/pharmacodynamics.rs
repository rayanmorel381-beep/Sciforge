pub fn emax_model(e0: f64, emax: f64, c: f64, ec50: f64) -> f64 {
    e0 + emax * c / (ec50 + c)
}

pub fn sigmoid_emax(e0: f64, emax: f64, c: f64, ec50: f64, n: f64) -> f64 {
    let cn = c.powf(n);
    e0 + emax * cn / (ec50.powf(n) + cn)
}

pub fn log_logistic(c: f64, ec50: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (ec50 / c).powf(slope))
}

pub fn therapeutic_index(td50: f64, ed50: f64) -> f64 {
    td50 / ed50
}

pub fn dose_response_hill(dose: f64, dmax: f64, ec50: f64, n: f64) -> f64 {
    let dn = dose.powf(n);
    dmax * dn / (ec50.powf(n) + dn)
}

pub fn competitive_antagonism(agonist: f64, ec50: f64, antagonist: f64, kb: f64) -> f64 {
    let apparent_ec50 = ec50 * (1.0 + antagonist / kb);
    agonist / (apparent_ec50 + agonist)
}

pub fn schild_equation(dose_ratio: f64, antagonist: f64) -> f64 {
    antagonist / (dose_ratio - 1.0)
}

pub fn receptor_occupancy(l: f64, kd: f64) -> f64 {
    l / (kd + l)
}

pub fn clark_equation(l: f64, kd: f64, emax: f64) -> f64 {
    emax * l / (kd + l)
}

pub fn operational_model(l: f64, kd: f64, tau: f64, n: f64, emax: f64) -> f64 {
    let tau_ln = tau.powf(n) * l.powf(n);
    emax * tau_ln / (tau_ln + (l + kd).powf(n))
}

pub fn imax_model(i0: f64, imax: f64, c: f64, ic50: f64) -> f64 {
    i0 * (1.0 - imax * c / (ic50 + c))
}

pub fn combination_index(d1: f64, dx1: f64, d2: f64, dx2: f64) -> f64 {
    d1 / dx1 + d2 / dx2
}

pub fn non_competitive_antagonism(
    agonist: f64,
    ec50: f64,
    antagonist: f64,
    kb: f64,
    emax: f64,
) -> f64 {
    let reduced_emax = emax / (1.0 + antagonist / kb);
    reduced_emax * agonist / (ec50 + agonist)
}

pub fn irreversible_antagonism(agonist: f64, ec50: f64, fraction_remaining: f64, emax: f64) -> f64 {
    let effective_emax = emax * fraction_remaining;
    effective_emax * agonist / (ec50 + agonist)
}

pub fn allosteric_modulator(
    agonist: f64,
    ec50: f64,
    modulator: f64,
    alpha: f64,
    beta: f64,
    km: f64,
    emax: f64,
) -> f64 {
    let mod_factor = 1.0 + modulator / km;
    let eff_ec50 = ec50 / (alpha * mod_factor);
    let eff_emax = emax * beta * mod_factor / (1.0 + beta * modulator / km);
    eff_emax * agonist / (eff_ec50 + agonist)
}

pub fn patlak_plot_slope(plasma_integral: f64, plasma_conc: f64, tissue_conc: f64) -> f64 {
    if plasma_conc < 1e-30 {
        return 0.0;
    }
    tissue_conc / plasma_conc - plasma_integral / plasma_conc
}

pub fn two_state_receptor(l: f64, kd_active: f64, kd_inactive: f64, l0: f64) -> f64 {
    let r_active_free = l0 / (1.0 + kd_active / l);
    let r_inactive_free = 1.0 / (1.0 + kd_inactive / l);
    r_active_free / (r_active_free + r_inactive_free)
}

pub fn partial_agonist_effect(l: f64, kd: f64, intrinsic_efficacy: f64, emax: f64) -> f64 {
    emax * intrinsic_efficacy * l / (kd + l)
}

pub fn inverse_agonist_effect(e0: f64, l: f64, kd: f64, neg_efficacy: f64) -> f64 {
    e0 * (1.0 - neg_efficacy * l / (kd + l))
}

pub fn biased_agonism_ratio(e1: f64, ec50_1: f64, e2: f64, ec50_2: f64) -> f64 {
    (e1 / ec50_1) / (e2 / ec50_2)
}

pub fn pk_pd_effect_compartment(ce: f64, emax: f64, ec50: f64, n: f64) -> f64 {
    let cn = ce.powf(n);
    emax * cn / (ec50.powf(n) + cn)
}

pub fn hysteresis_collapse_ke0(plasma: f64, effect_prev: f64, ke0: f64, dt: f64) -> f64 {
    effect_prev + ke0 * (plasma - effect_prev) * dt
}

pub fn tolerance_factor(exposure_time: f64, tolerance_rate: f64) -> f64 {
    (-tolerance_rate * exposure_time).exp()
}
