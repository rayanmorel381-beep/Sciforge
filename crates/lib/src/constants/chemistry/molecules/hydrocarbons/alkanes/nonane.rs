use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H20",
        name: "nonane",
        composition: &[(6, 9), (1, 20)],
        molar_mass: 128.255,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(219.6),
        boiling_point_k: Some(423.97),
        density_g_cm3: Some(0.718),
    }
}
