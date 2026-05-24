use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12O2",
        name: "ethyl butyrate",
        composition: &[(6, 6), (1, 12), (8, 2)],
        molar_mass: 116.160,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(180.0),
        boiling_point_k: Some(394.0),
        density_g_cm3: Some(0.879),
    }
}
