use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H10",
        name: "fluorene",
        composition: &[(6, 13), (1, 10)],
        molar_mass: 166.222,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(387.9),
        boiling_point_k: Some(568.0),
        density_g_cm3: Some(1.202),
    }
}
