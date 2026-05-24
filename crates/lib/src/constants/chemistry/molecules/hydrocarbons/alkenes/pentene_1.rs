use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10",
        name: "1-pentene",
        composition: &[(6, 5), (1, 10)],
        molar_mass: 70.133,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(165.2),
        boiling_point_k: Some(303.11),
        density_g_cm3: Some(0.640),
    }
}
