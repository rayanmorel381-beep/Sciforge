use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5NO2",
        name: "glycine",
        composition: &[(6, 2), (1, 5), (7, 1), (8, 2)],
        molar_mass: 75.067,
        category: "biomolecule",
        state_at_stp: "solid",
        melting_point_k: Some(506.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.607),
    }
}
