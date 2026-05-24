use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H32O2",
        name: "linoleic acid",
        composition: &[(6, 18), (1, 32), (8, 2)],
        molar_mass: 280.452,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(268.0),
        boiling_point_k: Some(503.0),
        density_g_cm3: Some(0.900),
    }
}
