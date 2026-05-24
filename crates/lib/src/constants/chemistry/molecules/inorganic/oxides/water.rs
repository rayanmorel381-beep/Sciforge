use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2O",
        name: "water",
        composition: &[(1, 2), (8, 1)],
        molar_mass: 18.015,
        category: "inorganic",
        state_at_stp: "liquid",
        melting_point_k: Some(273.15),
        boiling_point_k: Some(373.15),
        density_g_cm3: Some(0.997),
    }
}
