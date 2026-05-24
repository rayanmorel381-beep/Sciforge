use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "FeO",
        name: "iron(II) oxide",
        composition: &[(8, 1), (26, 1)],
        molar_mass: 71.844,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(1650.0),
        boiling_point_k: None,
        density_g_cm3: Some(5.745),
    }
}
