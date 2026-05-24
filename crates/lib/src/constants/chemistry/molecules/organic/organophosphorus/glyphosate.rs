use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H8NO5P",
        name: "glyphosate",
        composition: &[(6, 3), (1, 8), (7, 1), (8, 5), (15, 1)],
        molar_mass: 169.073,
        category: "organophosphonate",
        state_at_stp: "solid",
        melting_point_k: Some(457.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.704),
    }
}
