use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HgS",
        name: "cinnabar",
        composition: &[(16, 1), (80, 1)],
        molar_mass: 232.660,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(8.100),
    }
}
