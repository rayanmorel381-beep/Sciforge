use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2SO4",
        name: "sulfuric acid",
        composition: &[(1, 2), (16, 1), (8, 4)],
        molar_mass: 98.072,
        category: "inorganic",
        state_at_stp: "liquid",
        melting_point_k: Some(283.46),
        boiling_point_k: Some(610.0),
        density_g_cm3: Some(1.8302),
    }
}
