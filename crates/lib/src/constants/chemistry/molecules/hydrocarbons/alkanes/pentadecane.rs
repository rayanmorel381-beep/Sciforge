use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C15H32",
        name: "pentadecane",
        composition: &[(6, 15), (1, 32)],
        molar_mass: 212.415,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(283.0),
        boiling_point_k: Some(543.8),
        density_g_cm3: Some(0.769),
    }
}
