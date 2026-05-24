use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3Cl",
        name: "chloromethane",
        composition: &[(6, 1), (1, 3), (17, 1)],
        molar_mass: 50.488,
        category: "haloalkane",
        state_at_stp: "gas",
        melting_point_k: Some(175.4),
        boiling_point_k: Some(249.0),
        density_g_cm3: Some(0.911),
    }
}
