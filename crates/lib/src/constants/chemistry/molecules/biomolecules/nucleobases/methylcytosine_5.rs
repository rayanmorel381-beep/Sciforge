use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H7N3O",
        name: "5-methylcytosine",
        composition: &[(6, 5), (1, 7), (7, 3), (8, 1)],
        molar_mass: 125.129,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(543.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.4),
    }
}
