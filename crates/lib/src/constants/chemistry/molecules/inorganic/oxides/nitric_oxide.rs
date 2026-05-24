use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NO",
        name: "nitric oxide",
        composition: &[(7, 1), (8, 1)],
        molar_mass: 30.006,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(109.5),
        boiling_point_k: Some(121.4),
        density_g_cm3: None,
    }
}
