use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H27N7O14P2",
        name: "nicotinamide adenine dinucleotide",
        composition: &[(6, 21), (1, 27), (7, 7), (8, 14), (15, 2)],
        molar_mass: 663.425,
        category: "coenzyme",
        state_at_stp: "solid",
        melting_point_k: Some(413.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.700),
    }
}
