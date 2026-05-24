use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H5N",
        name: "pyridine",
        composition: &[(6, 5), (1, 5), (7, 1)],
        molar_mass: 79.100,
        category: "heterocycle",
        state_at_stp: "liquid",
        melting_point_k: Some(231.5),
        boiling_point_k: Some(388.4),
        density_g_cm3: Some(0.9819),
    }
}
