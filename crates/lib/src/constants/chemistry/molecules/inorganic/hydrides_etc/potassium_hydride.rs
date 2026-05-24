use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KH",
        name: "potassium hydride",
        composition: &[(1, 1), (19, 1)],
        molar_mass: 40.106,
        category: "hydride",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.430),
    }
}
