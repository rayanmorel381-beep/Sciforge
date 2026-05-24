use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H11NO3",
        name: "noradrenaline",
        composition: &[(6, 8), (1, 11), (7, 1), (8, 3)],
        molar_mass: 169.180,
        category: "hormone",
        state_at_stp: "solid",
        melting_point_k: Some(489.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.397),
    }
}
