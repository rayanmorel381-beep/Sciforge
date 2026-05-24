use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H42",
        name: "icosane",
        composition: &[(6, 20), (1, 42)],
        molar_mass: 282.547,
        category: "alkane",
        state_at_stp: "solid",
        melting_point_k: Some(309.8),
        boiling_point_k: Some(616.93),
        density_g_cm3: Some(0.789),
    }
}
