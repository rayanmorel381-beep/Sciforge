use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H32O2",
        name: "arachidonic acid",
        composition: &[(6, 20), (1, 32), (8, 2)],
        molar_mass: 304.467,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(224.2),
        boiling_point_k: Some(442.0),
        density_g_cm3: Some(0.922),
    }
}
