use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NH3",
        name: "ammonia",
        composition: &[(7, 1), (1, 3)],
        molar_mass: 17.031,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(195.42),
        boiling_point_k: Some(239.81),
        density_g_cm3: Some(0.000_769),
    }
}
