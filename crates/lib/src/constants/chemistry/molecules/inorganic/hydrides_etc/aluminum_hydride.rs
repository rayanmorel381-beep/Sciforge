use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "AlH3",
        name: "aluminum hydride",
        composition: &[(1, 3), (13, 1)],
        molar_mass: 30.005,
        category: "hydride",
        state_at_stp: "solid",
        melting_point_k: Some(423.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.486),
    }
}
