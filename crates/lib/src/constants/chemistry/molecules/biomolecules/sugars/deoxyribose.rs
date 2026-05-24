use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10O4",
        name: "deoxyribose",
        composition: &[(6, 5), (1, 10), (8, 4)],
        molar_mass: 134.131,
        category: "monosaccharide",
        state_at_stp: "solid",
        melting_point_k: Some(364.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.430),
    }
}
