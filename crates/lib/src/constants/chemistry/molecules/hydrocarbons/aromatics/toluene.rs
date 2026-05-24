use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H8",
        name: "toluene",
        composition: &[(6, 7), (1, 8)],
        molar_mass: 92.141,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(178.0),
        boiling_point_k: Some(383.75),
        density_g_cm3: Some(0.8669),
    }
}
