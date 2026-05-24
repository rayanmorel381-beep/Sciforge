use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "O2",
        name: "dioxygen",
        composition: &[(8, 2)],
        molar_mass: 31.998,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(54.36),
        boiling_point_k: Some(90.19),
        density_g_cm3: Some(0.001_429),
    }
}
