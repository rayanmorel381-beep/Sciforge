pub fn henderson_hasselbalch(pka: f64, base: f64, acid: f64) -> f64 {
    pka + (base / acid.max(1e-30)).log10()
}

pub fn pka_from_ka(ka: f64) -> f64 {
    -ka.max(1e-30).log10()
}

pub fn ka_from_pka(pka: f64) -> f64 {
    10.0_f64.powf(-pka)
}

pub fn pkb_from_pka(pka: f64, pkw: f64) -> f64 {
    pkw - pka
}

pub fn ph_strong_acid(concentration: f64) -> f64 {
    -concentration.max(1e-30).log10()
}

pub fn ph_strong_base(concentration: f64) -> f64 {
    14.0 + concentration.max(1e-30).log10()
}

pub fn ph_weak_acid(ka: f64, c: f64) -> f64 {
    0.5 * (-ka.max(1e-30).log10() - c.max(1e-30).log10())
}

pub fn ph_weak_base(kb: f64, c: f64) -> f64 {
    14.0 - 0.5 * (-kb.max(1e-30).log10() - c.max(1e-30).log10())
}

pub fn alpha_fraction(h: f64, ka_values: &[f64], species_index: usize) -> f64 {
    let mut denominator = 0.0;
    for j in 0..=ka_values.len() {
        let mut term = 1.0;
        for (i, &ka) in ka_values.iter().enumerate() {
            if i < j {
                term *= ka;
            } else {
                term *= h;
            }
        }
        denominator += term;
    }
    let mut numerator = 1.0;
    for (i, &ka) in ka_values.iter().enumerate() {
        if i < species_index {
            numerator *= ka;
        } else {
            numerator *= h;
        }
    }
    numerator / denominator.max(1e-30)
}

pub fn amphiprotic_ph(pka1: f64, pka2: f64) -> f64 {
    (pka1 + pka2) / 2.0
}

pub fn ionic_product_water(t: f64) -> f64 {
    let t_ref = 298.15;
    let delta_h = 55900.0;
    1e-14 * (delta_h / crate::constants::R_GAS * (1.0 / t_ref - 1.0 / t)).exp()
}
