use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CuSO4",
        name: "copper(II) sulfate",
        composition: &[(8, 4), (16, 1), (29, 1)],
        molar_mass: 159.609,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(383.0),
        boiling_point_k: None,
        density_g_cm3: Some(3.600),
    }
}
