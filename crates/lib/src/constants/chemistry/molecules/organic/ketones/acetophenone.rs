use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H8O",
        name: "acetophenone",
        composition: &[(6, 8), (1, 8), (8, 1)],
        molar_mass: 120.151,
        category: "ketone",
        state_at_stp: "liquid",
        melting_point_k: Some(292.7),
        boiling_point_k: Some(475.0),
        density_g_cm3: Some(1.028),
    }
}
