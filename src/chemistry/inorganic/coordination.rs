pub fn effective_atomic_number(metal_electrons: u32, ligand_electrons: u32) -> u32 {
    metal_electrons + ligand_electrons
}

pub fn chelate_effect(k_mono: f64, k_chelate: f64) -> f64 {
    (k_chelate / k_mono).log10()
}

pub fn irving_williams_stability(ionization_energy: f64, ionic_radius: f64) -> f64 {
    ionization_energy / ionic_radius.max(1e-30)
}

pub fn coordination_number_radius_ratio(r_cation: f64, r_anion: f64) -> u32 {
    let ratio = r_cation / r_anion.max(1e-30);
    if ratio < 0.155 {
        2
    } else if ratio < 0.225 {
        3
    } else if ratio < 0.414 {
        4
    } else if ratio < 0.732 {
        6
    } else {
        8
    }
}
