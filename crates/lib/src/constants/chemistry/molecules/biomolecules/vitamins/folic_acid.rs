use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C19H19N7O6",
        name: "folic acid",
        composition: &[(6, 19), (1, 19), (7, 7), (8, 6)],
        molar_mass: 441.404,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(523.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.600),
    }
}
