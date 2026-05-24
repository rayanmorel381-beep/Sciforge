use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C27H44O",
        name: "cholecalciferol",
        composition: &[(6, 27), (1, 44), (8, 1)],
        molar_mass: 384.638,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(357.0),
        boiling_point_k: Some(769.0),
        density_g_cm3: Some(0.971),
    }
}
