use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5CHO",
        name: "benzaldehyde",
        composition: &[(6, 7), (1, 6), (8, 1)],
        molar_mass: 106.124,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(217.0),
        boiling_point_k: Some(452.0),
        density_g_cm3: Some(1.044),
    }
}
