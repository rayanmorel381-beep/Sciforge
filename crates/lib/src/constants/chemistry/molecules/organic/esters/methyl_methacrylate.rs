use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H8O2",
        name: "methyl methacrylate",
        composition: &[(6, 5), (1, 8), (8, 2)],
        molar_mass: 100.117,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(225.0),
        boiling_point_k: Some(374.0),
        density_g_cm3: Some(0.940),
    }
}
