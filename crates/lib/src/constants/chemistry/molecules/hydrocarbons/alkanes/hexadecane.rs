use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H34",
        name: "hexadecane",
        composition: &[(6, 16), (1, 34)],
        molar_mass: 226.441,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(291.3),
        boiling_point_k: Some(559.95),
        density_g_cm3: Some(0.773),
    }
}
