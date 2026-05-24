use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H18O2",
        name: "ibuprofen",
        composition: &[(6, 13), (1, 18), (8, 2)],
        molar_mass: 206.285,
        category: "pharmaceutical",
        state_at_stp: "solid",
        melting_point_k: Some(349.0),
        boiling_point_k: Some(430.0),
        density_g_cm3: Some(1.029),
    }
}
