use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7NO",
        name: "N,N-dimethylformamide",
        composition: &[(6, 3), (1, 7), (7, 1), (8, 1)],
        molar_mass: 73.094,
        category: "amide",
        state_at_stp: "liquid",
        melting_point_k: Some(212.0),
        boiling_point_k: Some(426.0),
        density_g_cm3: Some(0.944),
    }
}
