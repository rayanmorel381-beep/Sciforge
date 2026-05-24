use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCHO",
        name: "formaldehyde",
        composition: &[(6, 1), (1, 2), (8, 1)],
        molar_mass: 30.026,
        category: "aldehyde",
        state_at_stp: "gas",
        melting_point_k: Some(181.0),
        boiling_point_k: Some(254.0),
        density_g_cm3: Some(0.815),
    }
}
