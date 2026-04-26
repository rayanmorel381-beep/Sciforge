pub fn radioactive_decay(n0: f64, lambda: f64, t: f64) -> f64 {
    n0 * (-lambda * t).exp()
}

pub fn half_life(lambda: f64) -> f64 {
    (2.0_f64).ln() / lambda
}

pub fn decay_constant(half_life: f64) -> f64 {
    (2.0_f64).ln() / half_life
}

pub fn age_from_ratio(ratio_daughter_parent: f64, lambda: f64) -> f64 {
    (1.0 + ratio_daughter_parent).ln() / lambda
}

pub fn carbon14_age(ratio: f64) -> f64 {
    -crate::constants::C14_MEAN_LIFE * ratio.ln()
}

pub fn potassium_argon_age(ar40: f64, k40: f64) -> f64 {
    use crate::constants::{K40_BRANCH_RATIO_AR, LAMBDA_K40_TOTAL};
    (1.0 + ar40 / (K40_BRANCH_RATIO_AR * k40)).ln() / LAMBDA_K40_TOTAL
}

pub fn uranium_lead_age(pb206: f64, u238: f64) -> f64 {
    (1.0 + pb206 / u238).ln() / crate::constants::LAMBDA_U238
}

pub fn isochron_age(slope: f64, lambda: f64) -> f64 {
    (1.0 + slope).ln() / lambda
}

pub fn fission_track_age(rho_s: f64, rho_i: f64, rho_d: f64, lambda: f64) -> f64 {
    (1.0 + lambda * rho_s * rho_d / rho_i).ln() / lambda
}

pub fn luminescence_dose(natural_signal: f64, dose_rate: f64) -> f64 {
    natural_signal / dose_rate
}

pub fn cosmogenic_exposure_age(concentration: f64, production_rate: f64, lambda: f64) -> f64 {
    -(1.0 - concentration * lambda / production_rate).ln() / lambda
}

pub fn uranium_235_lead_age(pb207: f64, u235: f64) -> f64 {
    (1.0 + pb207 / u235).ln() / crate::constants::LAMBDA_U235
}

pub fn concordia_u238_pb206(t: f64) -> f64 {
    (crate::constants::LAMBDA_U238 * t).exp() - 1.0
}

pub fn concordia_u235_pb207(t: f64) -> f64 {
    (crate::constants::LAMBDA_U235 * t).exp() - 1.0
}

pub fn concordia_age(pb206_u238: f64, pb207_u235: f64) -> f64 {
    let mut t = 1.0e9;
    for _ in 0..100 {
        let f238 = (crate::constants::LAMBDA_U238 * t).exp() - 1.0 - pb206_u238;
        let f235 = (crate::constants::LAMBDA_U235 * t).exp() - 1.0 - pb207_u235;
        let df238 = crate::constants::LAMBDA_U238 * (crate::constants::LAMBDA_U238 * t).exp();
        let df235 = crate::constants::LAMBDA_U235 * (crate::constants::LAMBDA_U235 * t).exp();
        let dt = (f238 / df238 + f235 / df235) / 2.0;
        t -= dt;
        if dt.abs() < 1.0 {
            break;
        }
    }
    t
}

pub fn thorium_232_lead_age(pb208: f64, th232: f64) -> f64 {
    (1.0 + pb208 / th232).ln() / crate::constants::LAMBDA_TH232
}

pub fn u_th_he_age(he4: f64, u238: f64, u235: f64, th232: f64) -> f64 {
    let mut t = 1.0e6;
    for _ in 0..100 {
        let he_produced = 8.0 * u238 * ((crate::constants::LAMBDA_U238 * t).exp() - 1.0)
            + 7.0 * u235 * ((crate::constants::LAMBDA_U235 * t).exp() - 1.0)
            + 6.0 * th232 * ((crate::constants::LAMBDA_TH232 * t).exp() - 1.0);
        let dhe =
            8.0 * u238 * crate::constants::LAMBDA_U238 * (crate::constants::LAMBDA_U238 * t).exp()
                + 7.0
                    * u235
                    * crate::constants::LAMBDA_U235
                    * (crate::constants::LAMBDA_U235 * t).exp()
                + 6.0
                    * th232
                    * crate::constants::LAMBDA_TH232
                    * (crate::constants::LAMBDA_TH232 * t).exp();
        let dt = (he_produced - he4) / dhe;
        t -= dt;
        if dt.abs() < 1.0 {
            break;
        }
    }
    t
}

pub fn radiogenic_heat_production(
    u238_ppm: f64,
    th232_ppm: f64,
    k40_ppm: f64,
    density: f64,
) -> f64 {
    use crate::constants::{HEAT_PRODUCTION_K40, HEAT_PRODUCTION_TH232, HEAT_PRODUCTION_U238};
    density
        * (u238_ppm * 1e-6 * HEAT_PRODUCTION_U238
            + th232_ppm * 1e-6 * HEAT_PRODUCTION_TH232
            + k40_ppm * 1e-6 * HEAT_PRODUCTION_K40)
}
