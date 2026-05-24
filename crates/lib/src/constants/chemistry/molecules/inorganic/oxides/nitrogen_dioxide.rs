use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NO2",
        name: "nitrogen dioxide",
        composition: &[(7, 1), (8, 2)],
        molar_mass: 46.005,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(261.95),
        boiling_point_k: Some(294.3),
        density_g_cm3: Some(1.443),
    }
}
