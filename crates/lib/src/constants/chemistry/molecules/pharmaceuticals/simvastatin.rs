use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C25H38O5",
        name: "simvastatin",
        composition: &[(1, 38), (6, 25), (8, 5)],
        molar_mass: 418.566,
        category: "statin",
        state_at_stp: "solid",
        melting_point_k: Some(412.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.180),
    }
}
