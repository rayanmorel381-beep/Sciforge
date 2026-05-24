use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H30O",
        name: "retinol",
        composition: &[(6, 20), (1, 30), (8, 1)],
        molar_mass: 286.452,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(335.0),
        boiling_point_k: Some(700.0),
        density_g_cm3: Some(0.954),
    }
}
