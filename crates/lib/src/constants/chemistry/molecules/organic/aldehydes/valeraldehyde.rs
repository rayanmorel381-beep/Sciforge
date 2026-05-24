use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9CHO",
        name: "valeraldehyde",
        composition: &[(6, 5), (1, 10), (8, 1)],
        molar_mass: 86.134,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(182.0),
        boiling_point_k: Some(376.0),
        density_g_cm3: Some(0.810),
    }
}
