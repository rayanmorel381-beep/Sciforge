use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H18FN3O3",
        name: "ciprofloxacin",
        composition: &[(1, 18), (6, 17), (7, 3), (8, 3), (9, 1)],
        molar_mass: 331.342,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(528.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.550),
    }
}
