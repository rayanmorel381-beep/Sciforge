use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H21N3O3",
        name: "pyrrolysine",
        composition: &[(6, 12), (1, 21), (7, 3), (8, 3)],
        molar_mass: 255.314,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.250),
    }
}
