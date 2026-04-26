pub fn strong_acid_strong_base_ph(c_acid: f64, v_acid: f64, c_base: f64, v_base: f64) -> f64 {
    let mol_acid = c_acid * v_acid;
    let mol_base = c_base * v_base;
    let v_total = v_acid + v_base;
    if mol_acid > mol_base {
        let excess = (mol_acid - mol_base) / v_total.max(1e-30);
        -excess.max(1e-30).log10()
    } else if mol_base > mol_acid {
        let excess = (mol_base - mol_acid) / v_total.max(1e-30);
        14.0 + excess.max(1e-30).log10()
    } else {
        7.0
    }
}

pub fn weak_acid_strong_base_ph(
    c_acid: f64,
    v_acid: f64,
    ka: f64,
    c_base: f64,
    v_base: f64,
) -> f64 {
    let mol_acid = c_acid * v_acid;
    let mol_base = c_base * v_base;
    let v_total = v_acid + v_base;
    if mol_base >= mol_acid {
        let excess = (mol_base - mol_acid) / v_total.max(1e-30);
        if excess < 1e-30 {
            let pka = -ka.max(1e-30).log10();
            let c = mol_acid / v_total.max(1e-30);
            return 0.5 * (14.0 + pka + c.max(1e-30).log10());
        }
        return 14.0 + excess.max(1e-30).log10();
    }
    let remaining_acid = (mol_acid - mol_base) / v_total.max(1e-30);
    let conjugate_base = mol_base / v_total.max(1e-30);
    let pka = -ka.max(1e-30).log10();
    pka + (conjugate_base / remaining_acid.max(1e-30))
        .max(1e-30)
        .log10()
}

pub fn equivalence_point_volume(c_analyte: f64, v_analyte: f64, c_titrant: f64) -> f64 {
    c_analyte * v_analyte / c_titrant.max(1e-30)
}

pub fn half_equivalence_ph(pka: f64) -> f64 {
    pka
}

pub fn buffer_range(pka: f64) -> (f64, f64) {
    (pka - 1.0, pka + 1.0)
}

pub fn buffer_capacity_vanslyke(c_total: f64, ka: f64, h: f64) -> f64 {
    2.303 * c_total * ka * h / (ka + h).powi(2)
}

pub fn diprotic_ph_first_equiv(pka1: f64, pka2: f64) -> f64 {
    (pka1 + pka2) / 2.0
}

pub fn back_titration_moles(
    mol_excess_added: f64,
    c_back_titrant: f64,
    v_back_titrant: f64,
) -> f64 {
    mol_excess_added - c_back_titrant * v_back_titrant
}
