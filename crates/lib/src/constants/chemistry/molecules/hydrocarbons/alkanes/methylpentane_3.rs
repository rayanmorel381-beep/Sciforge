use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14",
        name: "3-methylpentane",
        composition: &[(6, 6), (1, 14)],
        molar_mass: 86.175,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(113.25),
        boiling_point_k: Some(336.42),
        density_g_cm3: Some(0.664),
    }
}
