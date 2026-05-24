use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe3O4",
        name: "magnetite",
        composition: &[(8, 4), (26, 3)],
        molar_mass: 231.533,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(1870.0),
        boiling_point_k: None,
        density_g_cm3: Some(5.170),
    }
}
