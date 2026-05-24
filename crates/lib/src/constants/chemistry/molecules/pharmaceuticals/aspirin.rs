use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H8O4",
        name: "aspirin",
        composition: &[(6, 9), (1, 8), (8, 4)],
        molar_mass: 180.159,
        category: "pharmaceutical",
        state_at_stp: "solid",
        melting_point_k: Some(408.0),
        boiling_point_k: Some(413.0),
        density_g_cm3: Some(1.40),
    }
}
