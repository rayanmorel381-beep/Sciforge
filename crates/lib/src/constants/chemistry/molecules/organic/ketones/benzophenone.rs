use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H10O",
        name: "benzophenone",
        composition: &[(6, 13), (1, 10), (8, 1)],
        molar_mass: 182.222,
        category: "ketone",
        state_at_stp: "solid",
        melting_point_k: Some(321.0),
        boiling_point_k: Some(578.0),
        density_g_cm3: Some(1.110),
    }
}
