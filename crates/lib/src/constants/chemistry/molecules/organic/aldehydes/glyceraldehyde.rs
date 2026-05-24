use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H6O3",
        name: "glyceraldehyde",
        composition: &[(6, 3), (1, 6), (8, 3)],
        molar_mass: 90.078,
        category: "aldehyde",
        state_at_stp: "solid",
        melting_point_k: Some(418.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.455),
    }
}
