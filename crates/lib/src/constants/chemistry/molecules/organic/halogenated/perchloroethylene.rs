use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2Cl4",
        name: "perchloroethylene",
        composition: &[(6, 2), (17, 4)],
        molar_mass: 165.823,
        category: "haloalkene",
        state_at_stp: "liquid",
        melting_point_k: Some(250.8),
        boiling_point_k: Some(394.3),
        density_g_cm3: Some(1.622),
    }
}
