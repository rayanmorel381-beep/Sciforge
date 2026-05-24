use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H21NO3",
        name: "salbutamol",
        composition: &[(1, 21), (6, 13), (7, 1), (8, 3)],
        molar_mass: 239.311,
        category: "bronchodilator",
        state_at_stp: "solid",
        melting_point_k: Some(424.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.220),
    }
}
