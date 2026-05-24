use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H8O",
        name: "tetrahydrofuran",
        composition: &[(6, 4), (1, 8), (8, 1)],
        molar_mass: 72.107,
        category: "ether",
        state_at_stp: "liquid",
        melting_point_k: Some(164.7),
        boiling_point_k: Some(339.0),
        density_g_cm3: Some(0.886),
    }
}
