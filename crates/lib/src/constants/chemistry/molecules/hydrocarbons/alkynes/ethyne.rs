use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H2",
        name: "ethyne",
        composition: &[(6, 2), (1, 2)],
        molar_mass: 26.038,
        category: "alkyne",
        state_at_stp: "gas",
        melting_point_k: Some(192.4),
        boiling_point_k: Some(188.4),
        density_g_cm3: Some(0.001097),
    }
}
