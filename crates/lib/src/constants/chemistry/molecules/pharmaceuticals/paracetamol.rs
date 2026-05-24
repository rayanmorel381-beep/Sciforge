use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H9NO2",
        name: "paracetamol",
        composition: &[(6, 8), (1, 9), (7, 1), (8, 2)],
        molar_mass: 151.165,
        category: "pharmaceutical",
        state_at_stp: "solid",
        melting_point_k: Some(442.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.263),
    }
}
