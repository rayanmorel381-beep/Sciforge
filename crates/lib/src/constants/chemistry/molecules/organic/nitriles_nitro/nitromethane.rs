use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3NO2",
        name: "nitromethane",
        composition: &[(6, 1), (1, 3), (7, 1), (8, 2)],
        molar_mass: 61.040,
        category: "nitro",
        state_at_stp: "liquid",
        melting_point_k: Some(244.6),
        boiling_point_k: Some(374.4),
        density_g_cm3: Some(1.1371),
    }
}
