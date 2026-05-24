use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH4",
        name: "methane",
        composition: &[(6, 1), (1, 4)],
        molar_mass: 16.043,
        category: "alkane",
        state_at_stp: "gas",
        melting_point_k: Some(90.7),
        boiling_point_k: Some(111.66),
        density_g_cm3: Some(0.000656),
    }
}
