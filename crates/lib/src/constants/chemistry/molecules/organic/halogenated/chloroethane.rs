use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5Cl",
        name: "chloroethane",
        composition: &[(6, 2), (1, 5), (17, 1)],
        molar_mass: 64.514,
        category: "haloalkane",
        state_at_stp: "gas",
        melting_point_k: Some(136.0),
        boiling_point_k: Some(285.4),
        density_g_cm3: Some(0.892),
    }
}
