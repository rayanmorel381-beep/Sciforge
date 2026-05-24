use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5CN",
        name: "benzonitrile",
        composition: &[(6, 7), (1, 5), (7, 1)],
        molar_mass: 103.122,
        category: "nitrile",
        state_at_stp: "liquid",
        melting_point_k: Some(260.4),
        boiling_point_k: Some(464.0),
        density_g_cm3: Some(1.0093),
    }
}
