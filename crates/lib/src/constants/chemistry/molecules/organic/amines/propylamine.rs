use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H9N",
        name: "propylamine",
        composition: &[(6, 3), (1, 9), (7, 1)],
        molar_mass: 59.111,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(190.0),
        boiling_point_k: Some(321.0),
        density_g_cm3: Some(0.7180),
    }
}
