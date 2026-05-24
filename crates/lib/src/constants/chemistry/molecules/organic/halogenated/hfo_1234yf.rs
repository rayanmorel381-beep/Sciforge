use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H2F4",
        name: "2,3,3,3-tetrafluoropropene",
        composition: &[(6, 3), (1, 2), (9, 4)],
        molar_mass: 114.041,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(120.0),
        boiling_point_k: Some(244.0),
        density_g_cm3: Some(1.094),
    }
}
