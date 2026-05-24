use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaNO3",
        name: "sodium nitrate",
        composition: &[(7, 1), (8, 3), (11, 1)],
        molar_mass: 84.995,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(581.0),
        boiling_point_k: Some(653.0),
        density_g_cm3: Some(2.257),
    }
}
