use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C30H50O",
        name: "lanosterol",
        composition: &[(6, 30), (1, 50), (8, 1)],
        molar_mass: 426.717,
        category: "sterol",
        state_at_stp: "solid",
        melting_point_k: Some(412.0),
        boiling_point_k: None,
        density_g_cm3: Some(0.978),
    }
}
