use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6O2",
        name: "ethylene glycol",
        composition: &[(6, 2), (1, 6), (8, 2)],
        molar_mass: 62.068,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(260.2),
        boiling_point_k: Some(470.5),
        density_g_cm3: Some(1.1132),
    }
}
