use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H16NO2",
        name: "acetylcholine",
        composition: &[(6, 7), (1, 16), (7, 1), (8, 2)],
        molar_mass: 146.210,
        category: "neurotransmitter",
        state_at_stp: "solid",
        melting_point_k: Some(422.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.094),
    }
}
