use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cu2(CO3)(OH)2",
        name: "malachite",
        composition: &[(1, 2), (6, 1), (8, 5), (29, 2)],
        molar_mass: 221.116,
        category: "carbonate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(4.030),
    }
}
