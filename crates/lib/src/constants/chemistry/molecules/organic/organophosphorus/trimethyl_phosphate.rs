use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H9O4P",
        name: "trimethyl phosphate",
        composition: &[(6, 3), (1, 9), (8, 4), (15, 1)],
        molar_mass: 140.073,
        category: "organophosphate",
        state_at_stp: "liquid",
        melting_point_k: Some(227.0),
        boiling_point_k: Some(470.0),
        density_g_cm3: Some(1.214),
    }
}
