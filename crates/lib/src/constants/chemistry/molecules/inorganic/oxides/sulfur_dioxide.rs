use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "SO2",
        name: "sulfur dioxide",
        composition: &[(8, 2), (16, 1)],
        molar_mass: 64.066,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(200.95),
        boiling_point_k: Some(263.13),
        density_g_cm3: None,
    }
}
