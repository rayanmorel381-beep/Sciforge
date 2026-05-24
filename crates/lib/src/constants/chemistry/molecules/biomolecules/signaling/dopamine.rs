use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H11NO2",
        name: "dopamine",
        composition: &[(6, 8), (1, 11), (7, 1), (8, 2)],
        molar_mass: 153.181,
        category: "neurotransmitter",
        state_at_stp: "solid",
        melting_point_k: Some(401.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.260),
    }
}
