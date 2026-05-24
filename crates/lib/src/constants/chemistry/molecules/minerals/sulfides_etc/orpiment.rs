use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "As2S3",
        name: "orpiment",
        composition: &[(16, 3), (33, 2)],
        molar_mass: 246.041,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(573.0),
        boiling_point_k: Some(980.0),
        density_g_cm3: Some(3.490),
    }
}
