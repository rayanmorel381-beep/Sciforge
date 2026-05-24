use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6S",
        name: "dimethyl sulfide",
        composition: &[(6, 2), (1, 6), (16, 1)],
        molar_mass: 62.135,
        category: "thioether",
        state_at_stp: "liquid",
        melting_point_k: Some(174.9),
        boiling_point_k: Some(310.4),
        density_g_cm3: Some(0.846),
    }
}
