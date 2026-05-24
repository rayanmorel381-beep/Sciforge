use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C15H10Cl2N2O2",
        name: "lorazepam",
        composition: &[(1, 10), (6, 15), (7, 2), (8, 2), (17, 2)],
        molar_mass: 321.158,
        category: "benzodiazepine",
        state_at_stp: "solid",
        melting_point_k: Some(439.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.510),
    }
}
