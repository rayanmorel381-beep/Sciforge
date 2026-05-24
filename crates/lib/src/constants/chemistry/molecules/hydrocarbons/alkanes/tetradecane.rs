use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H30",
        name: "tetradecane",
        composition: &[(6, 14), (1, 30)],
        molar_mass: 198.388,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(278.9),
        boiling_point_k: Some(526.7),
        density_g_cm3: Some(0.763),
    }
}
