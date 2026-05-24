use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H28N7O17P3",
        name: "nicotinamide adenine dinucleotide phosphate",
        composition: &[(6, 21), (1, 28), (7, 7), (8, 17), (15, 3)],
        molar_mass: 743.405,
        category: "coenzyme",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.770),
    }
}
