use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H3NO",
        name: "oxazole",
        composition: &[(6, 3), (1, 3), (7, 1), (8, 1)],
        molar_mass: 69.062,
        category: "heterocycle",
        state_at_stp: "liquid",
        melting_point_k: Some(186.0),
        boiling_point_k: Some(343.5),
        density_g_cm3: Some(1.050),
    }
}
