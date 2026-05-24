use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H5N",
        name: "pyrrole",
        composition: &[(6, 4), (1, 5), (7, 1)],
        molar_mass: 67.090,
        category: "heterocycle",
        state_at_stp: "liquid",
        melting_point_k: Some(249.7),
        boiling_point_k: Some(402.9),
        density_g_cm3: Some(0.967),
    }
}
