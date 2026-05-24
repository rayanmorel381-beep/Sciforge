use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H30O2",
        name: "linolenic acid",
        composition: &[(6, 18), (1, 30), (8, 2)],
        molar_mass: 278.436,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(262.0),
        boiling_point_k: Some(503.0),
        density_g_cm3: Some(0.914),
    }
}
