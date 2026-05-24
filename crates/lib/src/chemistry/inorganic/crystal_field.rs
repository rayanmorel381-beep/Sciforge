pub fn crystal_field_splitting_octahedral(dq: f64) -> f64 {
    10.0 * dq
}

pub fn crystal_field_splitting_tetrahedral(dq_oct: f64) -> f64 {
    4.0 / 9.0 * dq_oct
}

pub fn cfse_octahedral(t2g: u32, eg: u32, dq: f64, pairing_energy: f64) -> f64 {
    let cfse = -0.4 * t2g as f64 * dq + 0.6 * eg as f64 * dq;
    let total_electrons = t2g + eg;
    let max_unpaired = if total_electrons <= 5 {
        total_electrons
    } else {
        10 - total_electrons
    };
    let actual_unpaired = t2g.min(3) + eg.min(2);
    let paired = if total_electrons > max_unpaired {
        (total_electrons - actual_unpaired.min(total_electrons)) as f64
    } else {
        0.0
    };
    cfse + paired * pairing_energy
}

pub fn magnetic_moment_spin_only(unpaired: u32) -> f64 {
    (unpaired as f64 * (unpaired as f64 + 2.0)).sqrt()
}

pub fn spectrochemical_series_dq(ligand_f: f64, metal_g: f64) -> f64 {
    ligand_f * metal_g
}

pub fn jahn_teller_distortion(eg_occupation: u32) -> bool {
    eg_occupation == 1 || eg_occupation == 3
}

pub fn nephelauxetic_ratio(b_complex: f64, b_free_ion: f64) -> f64 {
    b_complex / b_free_ion.max(1e-30)
}

pub fn racah_parameter_b(transitions: &[f64], dq: f64) -> f64 {
    if transitions.len() < 2 {
        return 0.0;
    }
    (transitions[1] - transitions[0] - 10.0 * dq) / 15.0
}
