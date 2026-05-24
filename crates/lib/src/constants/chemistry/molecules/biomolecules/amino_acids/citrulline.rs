use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H13N3O3",
        name: "citrulline",
        composition: &[(6, 6), (1, 13), (7, 3), (8, 3)],
        molar_mass: 175.187,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(495.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.293),
    }
}
