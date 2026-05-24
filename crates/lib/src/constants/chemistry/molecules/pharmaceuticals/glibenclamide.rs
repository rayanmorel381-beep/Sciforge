use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C23H28ClN3O5S",
        name: "glibenclamide",
        composition: &[(1, 28), (6, 23), (7, 3), (8, 5), (16, 1), (17, 1)],
        molar_mass: 494.004,
        category: "antidiabetic",
        state_at_stp: "solid",
        melting_point_k: Some(443.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.420),
    }
}
