use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H4N2",
        name: "imidazole",
        composition: &[(6, 3), (1, 4), (7, 2)],
        molar_mass: 68.077,
        category: "heterocycle",
        state_at_stp: "solid",
        melting_point_k: Some(363.0),
        boiling_point_k: Some(530.0),
        density_g_cm3: Some(1.030),
    }
}
