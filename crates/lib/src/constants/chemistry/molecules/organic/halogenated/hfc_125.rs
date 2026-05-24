use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2HF5",
        name: "pentafluoroethane",
        composition: &[(6, 2), (1, 1), (9, 5)],
        molar_mass: 120.022,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(170.0),
        boiling_point_k: Some(224.7),
        density_g_cm3: Some(1.245),
    }
}
