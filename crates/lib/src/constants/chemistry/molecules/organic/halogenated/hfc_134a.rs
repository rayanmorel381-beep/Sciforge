use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH2FCF3",
        name: "1,1,1,2-tetrafluoroethane",
        composition: &[(6, 2), (1, 2), (9, 4)],
        molar_mass: 102.030,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(172.0),
        boiling_point_k: Some(247.1),
        density_g_cm3: Some(1.225),
    }
}
