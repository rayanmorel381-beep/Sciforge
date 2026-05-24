use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cl2O",
        name: "dichlorine monoxide",
        composition: &[(8, 1), (17, 2)],
        molar_mass: 86.905,
        category: "oxide",
        state_at_stp: "gas",
        melting_point_k: Some(152.95),
        boiling_point_k: Some(275.0),
        density_g_cm3: None,
    }
}
