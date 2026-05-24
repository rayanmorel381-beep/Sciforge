use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H13OH",
        name: "hexan-1-ol",
        composition: &[(6, 6), (1, 14), (8, 1)],
        molar_mass: 102.177,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(228.6),
        boiling_point_k: Some(430.2),
        density_g_cm3: Some(0.814),
    }
}
