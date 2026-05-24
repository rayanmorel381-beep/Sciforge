use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H6",
        name: "benzene",
        composition: &[(6, 6), (1, 6)],
        molar_mass: 78.114,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(278.68),
        boiling_point_k: Some(353.24),
        density_g_cm3: Some(0.8765),
    }
}
