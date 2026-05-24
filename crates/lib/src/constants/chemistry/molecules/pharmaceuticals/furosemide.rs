use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H11ClN2O5S",
        name: "furosemide",
        composition: &[(1, 11), (6, 12), (7, 2), (8, 5), (16, 1), (17, 1)],
        molar_mass: 330.745,
        category: "diuretic",
        state_at_stp: "solid",
        melting_point_k: Some(479.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.660),
    }
}
