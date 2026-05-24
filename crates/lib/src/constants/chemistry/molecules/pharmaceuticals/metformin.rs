use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H11N5",
        name: "metformin",
        composition: &[(1, 11), (6, 4), (7, 5)],
        molar_mass: 129.164,
        category: "antidiabetic",
        state_at_stp: "solid",
        melting_point_k: Some(497.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
