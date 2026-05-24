use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CHCl3",
        name: "chloroform",
        composition: &[(6, 1), (1, 1), (17, 3)],
        molar_mass: 119.378,
        category: "haloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(209.7),
        boiling_point_k: Some(334.3),
        density_g_cm3: Some(1.483),
    }
}
