use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CCl4",
        name: "carbon tetrachloride",
        composition: &[(6, 1), (17, 4)],
        molar_mass: 153.823,
        category: "haloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(250.0),
        boiling_point_k: Some(349.8),
        density_g_cm3: Some(1.5867),
    }
}
