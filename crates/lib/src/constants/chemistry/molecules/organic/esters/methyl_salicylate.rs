use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H8O3",
        name: "methyl salicylate",
        composition: &[(6, 8), (1, 8), (8, 3)],
        molar_mass: 152.149,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(265.9),
        boiling_point_k: Some(495.0),
        density_g_cm3: Some(1.174),
    }
}
