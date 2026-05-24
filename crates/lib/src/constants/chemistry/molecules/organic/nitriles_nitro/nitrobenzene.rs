use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5NO2",
        name: "nitrobenzene",
        composition: &[(6, 6), (1, 5), (7, 1), (8, 2)],
        molar_mass: 123.110,
        category: "nitro",
        state_at_stp: "liquid",
        melting_point_k: Some(278.8),
        boiling_point_k: Some(484.0),
        density_g_cm3: Some(1.199),
    }
}
