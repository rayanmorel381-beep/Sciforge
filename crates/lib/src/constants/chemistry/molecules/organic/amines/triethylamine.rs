use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H15N",
        name: "triethylamine",
        composition: &[(6, 6), (1, 15), (7, 1)],
        molar_mass: 101.193,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(158.4),
        boiling_point_k: Some(361.9),
        density_g_cm3: Some(0.7255),
    }
}
