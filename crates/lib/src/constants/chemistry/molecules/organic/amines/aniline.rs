use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5NH2",
        name: "aniline",
        composition: &[(6, 6), (1, 7), (7, 1)],
        molar_mass: 93.129,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(267.0),
        boiling_point_k: Some(457.6),
        density_g_cm3: Some(1.0217),
    }
}
