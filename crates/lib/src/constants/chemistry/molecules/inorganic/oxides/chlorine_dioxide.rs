use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "ClO2",
        name: "chlorine dioxide",
        composition: &[(8, 2), (17, 1)],
        molar_mass: 67.452,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(214.05),
        boiling_point_k: Some(284.0),
        density_g_cm3: Some(1.642),
    }
}
