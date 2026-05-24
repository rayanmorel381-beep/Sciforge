use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H18",
        name: "1-nonene",
        composition: &[(6, 9), (1, 18)],
        molar_mass: 126.239,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(191.9),
        boiling_point_k: Some(419.86),
        density_g_cm3: Some(0.729),
    }
}
