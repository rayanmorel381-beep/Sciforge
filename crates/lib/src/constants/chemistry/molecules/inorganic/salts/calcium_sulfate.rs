use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaSO4",
        name: "calcium sulfate",
        composition: &[(8, 4), (16, 1), (20, 1)],
        molar_mass: 136.140,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1733.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.960),
    }
}
