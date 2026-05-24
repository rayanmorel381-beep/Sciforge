use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C22H24N2O8",
        name: "doxycycline",
        composition: &[(1, 24), (6, 22), (7, 2), (8, 8)],
        molar_mass: 444.435,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(474.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.550),
    }
}
