use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9NO",
        name: "N,N-dimethylacetamide",
        composition: &[(6, 4), (1, 9), (7, 1), (8, 1)],
        molar_mass: 87.121,
        category: "amide",
        state_at_stp: "liquid",
        melting_point_k: Some(253.0),
        boiling_point_k: Some(438.0),
        density_g_cm3: Some(0.940),
    }
}
