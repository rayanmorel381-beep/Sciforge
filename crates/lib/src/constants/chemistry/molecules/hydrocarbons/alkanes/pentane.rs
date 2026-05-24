use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H12",
        name: "pentane",
        composition: &[(6, 5), (1, 12)],
        molar_mass: 72.149,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(143.4),
        boiling_point_k: Some(309.21),
        density_g_cm3: Some(0.626),
    }
}
