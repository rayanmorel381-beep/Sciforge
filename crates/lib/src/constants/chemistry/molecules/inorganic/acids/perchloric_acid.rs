use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HClO4",
        name: "perchloric acid",
        composition: &[(1, 1), (8, 4), (17, 1)],
        molar_mass: 100.460,
        category: "acid",
        state_at_stp: "liquid",
        melting_point_k: Some(256.0),
        boiling_point_k: Some(476.0),
        density_g_cm3: Some(1.768),
    }
}
