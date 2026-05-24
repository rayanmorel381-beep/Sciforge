use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3CONH2",
        name: "acetamide",
        composition: &[(6, 2), (1, 5), (7, 1), (8, 1)],
        molar_mass: 59.068,
        category: "amide",
        state_at_stp: "solid",
        melting_point_k: Some(354.0),
        boiling_point_k: Some(494.3),
        density_g_cm3: Some(1.159),
    }
}
