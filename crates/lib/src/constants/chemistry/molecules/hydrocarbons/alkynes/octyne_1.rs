use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H14",
        name: "1-octyne",
        composition: &[(6, 8), (1, 14)],
        molar_mass: 110.198,
        category: "alkyne",
        state_at_stp: "liquid",
        melting_point_k: Some(193.5),
        boiling_point_k: Some(399.35),
        density_g_cm3: Some(0.747),
    }
}
