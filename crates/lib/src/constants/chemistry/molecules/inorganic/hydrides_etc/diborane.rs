use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "B2H6",
        name: "diborane",
        composition: &[(1, 6), (5, 2)],
        molar_mass: 27.670,
        category: "hydride",
        state_at_stp: "gas",
        melting_point_k: Some(108.5),
        boiling_point_k: Some(180.7),
        density_g_cm3: None,
    }
}
