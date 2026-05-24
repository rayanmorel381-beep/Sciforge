use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H19N3O4S",
        name: "ampicillin",
        composition: &[(1, 19), (6, 16), (7, 3), (8, 4), (16, 1)],
        molar_mass: 349.405,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(481.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.410),
    }
}
