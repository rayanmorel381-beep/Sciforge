use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH2Cl2",
        name: "dichloromethane",
        composition: &[(6, 1), (1, 2), (17, 2)],
        molar_mass: 84.933,
        category: "haloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(176.0),
        boiling_point_k: Some(312.8),
        density_g_cm3: Some(1.3266),
    }
}
