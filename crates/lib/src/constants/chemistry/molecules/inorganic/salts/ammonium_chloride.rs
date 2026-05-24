use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NH4Cl",
        name: "ammonium chloride",
        composition: &[(1, 4), (7, 1), (17, 1)],
        molar_mass: 53.491,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(611.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.527),
    }
}
