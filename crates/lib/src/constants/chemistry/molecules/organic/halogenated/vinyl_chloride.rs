use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H3Cl",
        name: "vinyl chloride",
        composition: &[(6, 2), (1, 3), (17, 1)],
        molar_mass: 62.498,
        category: "haloalkene",
        state_at_stp: "gas",
        melting_point_k: Some(119.0),
        boiling_point_k: Some(259.8),
        density_g_cm3: Some(0.911),
    }
}
