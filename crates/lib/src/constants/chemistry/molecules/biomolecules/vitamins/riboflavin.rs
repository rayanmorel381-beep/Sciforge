use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H20N4O6",
        name: "riboflavin",
        composition: &[(6, 17), (1, 20), (7, 4), (8, 6)],
        molar_mass: 376.369,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(563.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.650),
    }
}
