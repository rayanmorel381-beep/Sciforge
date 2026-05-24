use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H18N4O3",
        name: "trimethoprim",
        composition: &[(1, 18), (6, 14), (7, 4), (8, 3)],
        molar_mass: 290.323,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(472.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
