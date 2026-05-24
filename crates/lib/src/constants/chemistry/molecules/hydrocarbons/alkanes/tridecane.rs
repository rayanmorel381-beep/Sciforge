use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H28",
        name: "tridecane",
        composition: &[(6, 13), (1, 28)],
        molar_mass: 184.361,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(268.0),
        boiling_point_k: Some(508.6),
        density_g_cm3: Some(0.756),
    }
}
