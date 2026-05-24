use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H9N3O3",
        name: "metronidazole",
        composition: &[(1, 9), (6, 6), (7, 3), (8, 3)],
        molar_mass: 171.155,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(434.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.450),
    }
}
