use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C31H46O2",
        name: "phylloquinone",
        composition: &[(6, 31), (1, 46), (8, 2)],
        molar_mass: 450.707,
        category: "vitamin",
        state_at_stp: "liquid",
        melting_point_k: Some(253.0),
        boiling_point_k: Some(746.0),
        density_g_cm3: Some(0.984),
    }
}
