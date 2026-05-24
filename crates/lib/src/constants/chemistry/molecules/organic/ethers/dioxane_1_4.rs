use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H8O2",
        name: "1,4-dioxane",
        composition: &[(6, 4), (1, 8), (8, 2)],
        molar_mass: 88.106,
        category: "ether",
        state_at_stp: "liquid",
        melting_point_k: Some(284.9),
        boiling_point_k: Some(374.5),
        density_g_cm3: Some(1.033),
    }
}
