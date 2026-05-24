use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H4",
        name: "propyne",
        composition: &[(6, 3), (1, 4)],
        molar_mass: 40.064,
        category: "alkyne",
        state_at_stp: "gas",
        melting_point_k: Some(170.5),
        boiling_point_k: Some(250.0),
        density_g_cm3: Some(0.0017),
    }
}
