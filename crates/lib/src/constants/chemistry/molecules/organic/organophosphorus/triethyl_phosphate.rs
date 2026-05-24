use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H15O4P",
        name: "triethyl phosphate",
        composition: &[(6, 6), (1, 15), (8, 4), (15, 1)],
        molar_mass: 182.154,
        category: "organophosphate",
        state_at_stp: "liquid",
        melting_point_k: Some(217.0),
        boiling_point_k: Some(488.0),
        density_g_cm3: Some(1.072),
    }
}
