use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H22N4O3S",
        name: "ranitidine",
        composition: &[(1, 22), (6, 13), (7, 4), (8, 3), (16, 1)],
        molar_mass: 314.404,
        category: "h2_antagonist",
        state_at_stp: "solid",
        melting_point_k: Some(343.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.260),
    }
}
