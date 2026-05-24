use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H8O",
        name: "benzyl alcohol",
        composition: &[(6, 7), (1, 8), (8, 1)],
        molar_mass: 108.140,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(257.8),
        boiling_point_k: Some(478.6),
        density_g_cm3: Some(1.044),
    }
}
