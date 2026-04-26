pub fn one_compartment(dose: f64, vd: f64, ke: f64, t: f64) -> f64 {
    (dose / vd) * (-ke * t).exp()
}

pub fn one_compartment_iv_infusion(r0: f64, ke: f64, vd: f64, t: f64, t_inf: f64) -> f64 {
    if t <= t_inf {
        (r0 / (ke * vd)) * (1.0 - (-ke * t).exp())
    } else {
        let c_end = (r0 / (ke * vd)) * (1.0 - (-ke * t_inf).exp());
        c_end * (-ke * (t - t_inf)).exp()
    }
}

pub fn two_compartment(a: f64, alpha: f64, b: f64, beta: f64, t: f64) -> f64 {
    a * (-alpha * t).exp() + b * (-beta * t).exp()
}

pub fn oral_one_compartment(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64, t: f64) -> f64 {
    if (ka - ke).abs() < 1e-12 {
        (f_bio * dose * ka / vd) * t * (-ke * t).exp()
    } else {
        (f_bio * dose * ka / (vd * (ka - ke))) * ((-ke * t).exp() - (-ka * t).exp())
    }
}

pub fn clearance(ke: f64, vd: f64) -> f64 {
    ke * vd
}

pub fn half_life(ke: f64) -> f64 {
    (2.0_f64).ln() / ke
}

pub fn auc_iv_bolus(dose: f64, cl: f64) -> f64 {
    dose / cl
}

pub fn auc_trapezoidal(times: &[f64], concentrations: &[f64]) -> f64 {
    let n = times.len().min(concentrations.len());
    let mut area = 0.0;
    for i in 1..n {
        let dt = times[i] - times[i - 1];
        area += 0.5 * (concentrations[i - 1] + concentrations[i]) * dt;
    }
    area
}

pub fn bioavailability(auc_oral: f64, dose_oral: f64, auc_iv: f64, dose_iv: f64) -> f64 {
    (auc_oral * dose_iv) / (auc_iv * dose_oral)
}

pub fn volume_of_distribution(dose: f64, c0: f64) -> f64 {
    dose / c0
}

pub fn steady_state_concentration(dose: f64, cl: f64, tau: f64, f_bio: f64) -> f64 {
    f_bio * dose / (cl * tau)
}

pub fn loading_dose(css_target: f64, vd: f64, f_bio: f64) -> f64 {
    css_target * vd / f_bio
}

pub fn maintenance_dose(css_target: f64, cl: f64, tau: f64, f_bio: f64) -> f64 {
    css_target * cl * tau / f_bio
}

pub fn accumulation_factor(ke: f64, tau: f64) -> f64 {
    1.0 / (1.0 - (-ke * tau).exp())
}

pub fn tmax_oral(ka: f64, ke: f64) -> f64 {
    if (ka - ke).abs() < 1e-12 {
        return 1.0 / ke;
    }
    (ka / ke).ln() / (ka - ke)
}

pub fn cmax_oral(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64) -> f64 {
    let tmax = tmax_oral(ka, ke);
    oral_one_compartment(f_bio, dose, ka, ke, vd, tmax)
}

pub fn three_compartment(a: f64, alpha: f64, b: f64, beta: f64, c: f64, gamma: f64, t: f64) -> f64 {
    a * (-alpha * t).exp() + b * (-beta * t).exp() + c * (-gamma * t).exp()
}

pub fn multiple_dose_superposition(
    dose: f64,
    vd: f64,
    ke: f64,
    tau: f64,
    t: f64,
    n_doses: usize,
) -> f64 {
    let mut total = 0.0;
    for k in 0..n_doses {
        let tk = t - k as f64 * tau;
        if tk >= 0.0 {
            total += (dose / vd) * (-ke * tk).exp();
        }
    }
    total
}

pub fn css_max(dose: f64, vd: f64, ke: f64, tau: f64) -> f64 {
    (dose / vd) / (1.0 - (-ke * tau).exp())
}

pub fn css_min(dose: f64, vd: f64, ke: f64, tau: f64) -> f64 {
    css_max(dose, vd, ke, tau) * (-ke * tau).exp()
}

pub fn time_above_mic(dose: f64, vd: f64, ke: f64, mic: f64) -> f64 {
    let c0 = dose / vd;
    if c0 <= mic {
        return 0.0;
    }
    (c0 / mic).ln() / ke
}

pub fn hepatic_extraction_ratio(cl_hepatic: f64, q_hepatic: f64) -> f64 {
    cl_hepatic / q_hepatic
}

pub fn well_stirred_model(q_h: f64, fu: f64, cl_int: f64) -> f64 {
    q_h * fu * cl_int / (q_h + fu * cl_int)
}

pub fn renal_clearance(fraction_unbound: f64, gfr: f64) -> f64 {
    fraction_unbound * gfr
}

pub fn auc_log_trapezoidal(times: &[f64], concentrations: &[f64]) -> f64 {
    let n = times.len().min(concentrations.len());
    let mut area = 0.0;
    for i in 1..n {
        let dt = times[i] - times[i - 1];
        let c1 = concentrations[i - 1];
        let c2 = concentrations[i];
        if c1 > 0.0 && c2 > 0.0 && (c1 - c2).abs() > 1e-30 {
            area += dt * (c1 - c2) / (c1 / c2).ln();
        } else {
            area += 0.5 * (c1 + c2) * dt;
        }
    }
    area
}

pub fn mean_residence_time(aumc: f64, auc: f64) -> f64 {
    aumc / auc
}

pub fn aumc_trapezoidal(times: &[f64], concentrations: &[f64]) -> f64 {
    let n = times.len().min(concentrations.len());
    let mut area = 0.0;
    for i in 1..n {
        let dt = times[i] - times[i - 1];
        let tc1 = times[i - 1] * concentrations[i - 1];
        let tc2 = times[i] * concentrations[i];
        area += 0.5 * (tc1 + tc2) * dt;
    }
    area
}

pub fn flip_flop_kinetics(ka: f64, ke: f64) -> bool {
    ka < ke
}
