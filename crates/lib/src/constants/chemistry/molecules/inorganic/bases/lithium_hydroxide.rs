use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "LiOH",
        name: "lithium hydroxide",
        composition: &[(1, 1), (3, 1), (8, 1)],
        molar_mass: 23.948,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(735.0),
        boiling_point_k: Some(1899.0),
        density_g_cm3: Some(1.460),
    }
}
