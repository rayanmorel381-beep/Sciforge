use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cu(OH)2",
        name: "copper(II) hydroxide",
        composition: &[(1, 2), (8, 2), (29, 1)],
        molar_mass: 97.561,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.368),
    }
}
