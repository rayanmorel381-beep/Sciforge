use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9NO2",
        name: "gamma-aminobutyric acid",
        composition: &[(6, 4), (1, 9), (7, 1), (8, 2)],
        molar_mass: 103.120,
        category: "neurotransmitter",
        state_at_stp: "solid",
        melting_point_k: Some(476.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.110),
    }
}
