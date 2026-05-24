use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH4N2O",
        name: "urea",
        composition: &[(6, 1), (1, 4), (7, 2), (8, 1)],
        molar_mass: 60.056,
        category: "biomolecule",
        state_at_stp: "solid",
        melting_point_k: Some(406.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.32),
    }
}
