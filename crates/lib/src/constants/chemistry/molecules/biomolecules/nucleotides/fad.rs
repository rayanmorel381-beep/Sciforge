use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C27H33N9O15P2",
        name: "flavin adenine dinucleotide",
        composition: &[(6, 27), (1, 33), (7, 9), (8, 15), (15, 2)],
        molar_mass: 785.550,
        category: "coenzyme",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.650),
    }
}
