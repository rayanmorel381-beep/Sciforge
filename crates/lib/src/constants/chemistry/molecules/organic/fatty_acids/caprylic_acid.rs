use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H16O2",
        name: "caprylic acid",
        composition: &[(6, 8), (1, 16), (8, 2)],
        molar_mass: 144.211,
        category: "fatty acid",
        state_at_stp: "liquid",
        melting_point_k: Some(289.5),
        boiling_point_k: Some(512.0),
        density_g_cm3: Some(0.910),
    }
}
