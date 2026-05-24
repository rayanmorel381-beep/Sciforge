use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5OH",
        name: "phenol",
        composition: &[(6, 6), (1, 6), (8, 1)],
        molar_mass: 94.113,
        category: "alcohol",
        state_at_stp: "solid",
        melting_point_k: Some(313.9),
        boiling_point_k: Some(454.9),
        density_g_cm3: Some(1.07),
    }
}
