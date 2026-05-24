use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H3PO4",
        name: "phosphoric acid",
        composition: &[(1, 3), (8, 4), (15, 1)],
        molar_mass: 97.994,
        category: "acid",
        state_at_stp: "solid",
        melting_point_k: Some(315.5),
        boiling_point_k: Some(431.0),
        density_g_cm3: Some(1.685),
    }
}
