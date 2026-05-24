use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C19H40",
        name: "nonadecane",
        composition: &[(6, 19), (1, 40)],
        molar_mass: 268.521,
        category: "alkane",
        state_at_stp: "solid",
        melting_point_k: Some(305.2),
        boiling_point_k: Some(603.0),
        density_g_cm3: Some(0.785),
    }
}
