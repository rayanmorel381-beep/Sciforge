use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "N2O",
        name: "nitrous oxide",
        composition: &[(7, 2), (8, 1)],
        molar_mass: 44.013,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(182.3),
        boiling_point_k: Some(184.7),
        density_g_cm3: None,
    }
}
