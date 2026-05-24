use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KF",
        name: "potassium fluoride",
        composition: &[(9, 1), (19, 1)],
        molar_mass: 58.097,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1131.0),
        boiling_point_k: Some(1775.0),
        density_g_cm3: Some(2.480),
    }
}
