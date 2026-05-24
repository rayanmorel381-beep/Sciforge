use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H4N2",
        name: "pyrimidine",
        composition: &[(6, 4), (1, 4), (7, 2)],
        molar_mass: 80.088,
        category: "heterocycle",
        state_at_stp: "solid",
        melting_point_k: Some(295.4),
        boiling_point_k: Some(396.0),
        density_g_cm3: Some(1.016),
    }
}
