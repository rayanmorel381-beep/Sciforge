use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H12O",
        name: "methyl tert-butyl ether",
        composition: &[(6, 5), (1, 12), (8, 1)],
        molar_mass: 88.150,
        category: "ether",
        state_at_stp: "liquid",
        melting_point_k: Some(164.0),
        boiling_point_k: Some(328.3),
        density_g_cm3: Some(0.7404),
    }
}
