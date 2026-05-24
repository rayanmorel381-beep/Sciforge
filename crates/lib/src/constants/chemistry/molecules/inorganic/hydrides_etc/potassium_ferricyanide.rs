use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "K3[Fe(CN)6]",
        name: "potassium ferricyanide",
        composition: &[(6, 6), (7, 6), (19, 3), (26, 1)],
        molar_mass: 329.244,
        category: "complex",
        state_at_stp: "solid",
        melting_point_k: Some(573.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.890),
    }
}
