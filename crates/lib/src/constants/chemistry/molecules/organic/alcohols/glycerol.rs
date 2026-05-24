use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H8O3",
        name: "glycerol",
        composition: &[(6, 3), (1, 8), (8, 3)],
        molar_mass: 92.094,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(291.3),
        boiling_point_k: Some(563.0),
        density_g_cm3: Some(1.261),
    }
}
