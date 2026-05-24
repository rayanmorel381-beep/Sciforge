use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H11OH",
        name: "pentan-1-ol",
        composition: &[(6, 5), (1, 12), (8, 1)],
        molar_mass: 88.150,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(195.0),
        boiling_point_k: Some(411.0),
        density_g_cm3: Some(0.814),
    }
}
