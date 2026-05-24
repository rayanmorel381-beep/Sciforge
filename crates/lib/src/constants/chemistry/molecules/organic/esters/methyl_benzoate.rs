use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H8O2",
        name: "methyl benzoate",
        composition: &[(6, 8), (1, 8), (8, 2)],
        molar_mass: 136.150,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(260.7),
        boiling_point_k: Some(472.0),
        density_g_cm3: Some(1.094),
    }
}
