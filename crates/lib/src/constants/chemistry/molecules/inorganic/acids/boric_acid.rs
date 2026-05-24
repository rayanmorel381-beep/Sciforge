use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H3BO3",
        name: "boric acid",
        composition: &[(1, 3), (5, 1), (8, 3)],
        molar_mass: 61.833,
        category: "acid",
        state_at_stp: "solid",
        melting_point_k: Some(444.6),
        boiling_point_k: Some(573.0),
        density_g_cm3: Some(1.435),
    }
}
