use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H12",
        name: "neopentane",
        composition: &[(6, 5), (1, 12)],
        molar_mass: 72.149,
        category: "alkane",
        state_at_stp: "gas",
        melting_point_k: Some(256.65),
        boiling_point_k: Some(282.65),
        density_g_cm3: Some(0.585),
    }
}
