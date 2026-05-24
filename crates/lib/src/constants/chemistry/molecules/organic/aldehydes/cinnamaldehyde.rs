use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H8O",
        name: "cinnamaldehyde",
        composition: &[(6, 9), (1, 8), (8, 1)],
        molar_mass: 132.162,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(265.5),
        boiling_point_k: Some(521.0),
        density_g_cm3: Some(1.050),
    }
}
