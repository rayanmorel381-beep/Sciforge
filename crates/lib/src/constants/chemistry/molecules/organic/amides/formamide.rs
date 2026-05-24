use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCONH2",
        name: "formamide",
        composition: &[(6, 1), (1, 3), (7, 1), (8, 1)],
        molar_mass: 45.041,
        category: "amide",
        state_at_stp: "liquid",
        melting_point_k: Some(275.6),
        boiling_point_k: Some(483.0),
        density_g_cm3: Some(1.133),
    }
}
