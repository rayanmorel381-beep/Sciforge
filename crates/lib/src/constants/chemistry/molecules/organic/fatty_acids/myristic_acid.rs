use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H28O2",
        name: "myristic acid",
        composition: &[(6, 14), (1, 28), (8, 2)],
        molar_mass: 228.371,
        category: "fatty acid",
        state_at_stp: "solid",
        melting_point_k: Some(327.4),
        boiling_point_k: Some(599.0),
        density_g_cm3: Some(0.862),
    }
}
