use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H4",
        name: "ethene",
        composition: &[(6, 2), (1, 4)],
        molar_mass: 28.054,
        category: "alkene",
        state_at_stp: "gas",
        melting_point_k: Some(104.0),
        boiling_point_k: Some(169.4),
        density_g_cm3: Some(0.001178),
    }
}
