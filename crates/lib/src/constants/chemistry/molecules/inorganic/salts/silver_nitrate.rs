use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "AgNO3",
        name: "silver nitrate",
        composition: &[(7, 1), (8, 3), (47, 1)],
        molar_mass: 169.872,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(482.0),
        boiling_point_k: Some(717.0),
        density_g_cm3: Some(4.350),
    }
}
