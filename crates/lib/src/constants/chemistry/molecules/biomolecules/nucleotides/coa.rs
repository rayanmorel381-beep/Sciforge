use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H36N7O16P3S",
        name: "coenzyme A",
        composition: &[(6, 21), (1, 36), (7, 7), (8, 16), (15, 3), (16, 1)],
        molar_mass: 767.535,
        category: "coenzyme",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.910),
    }
}
