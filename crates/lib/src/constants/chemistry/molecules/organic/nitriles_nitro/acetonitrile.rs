use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3CN",
        name: "acetonitrile",
        composition: &[(6, 2), (1, 3), (7, 1)],
        molar_mass: 41.052,
        category: "nitrile",
        state_at_stp: "liquid",
        melting_point_k: Some(229.3),
        boiling_point_k: Some(354.8),
        density_g_cm3: Some(0.786),
    }
}
