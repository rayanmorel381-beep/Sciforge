use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H12",
        name: "1-heptyne",
        composition: &[(6, 7), (1, 12)],
        molar_mass: 96.171,
        category: "alkyne",
        state_at_stp: "liquid",
        melting_point_k: Some(192.0),
        boiling_point_k: Some(372.95),
        density_g_cm3: Some(0.733),
    }
}
