use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14",
        name: "hexane",
        composition: &[(6, 6), (1, 14)],
        molar_mass: 86.175,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(177.83),
        boiling_point_k: Some(341.88),
        density_g_cm3: Some(0.6606),
    }
}
