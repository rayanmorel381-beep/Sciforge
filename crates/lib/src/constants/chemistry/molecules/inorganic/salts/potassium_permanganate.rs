use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KMnO4",
        name: "potassium permanganate",
        composition: &[(8, 4), (19, 1), (25, 1)],
        molar_mass: 158.034,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(513.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.703),
    }
}
