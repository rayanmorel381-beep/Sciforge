use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H16",
        name: "1-octene",
        composition: &[(6, 8), (1, 16)],
        molar_mass: 112.213,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(171.4),
        boiling_point_k: Some(394.41),
        density_g_cm3: Some(0.715),
    }
}
