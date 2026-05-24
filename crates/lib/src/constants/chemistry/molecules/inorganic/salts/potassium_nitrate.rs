use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KNO3",
        name: "potassium nitrate",
        composition: &[(7, 1), (8, 3), (19, 1)],
        molar_mass: 101.103,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(607.0),
        boiling_point_k: Some(673.0),
        density_g_cm3: Some(2.109),
    }
}
