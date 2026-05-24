use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H7NO3S",
        name: "taurine",
        composition: &[(6, 2), (1, 7), (7, 1), (8, 3), (16, 1)],
        molar_mass: 125.147,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(601.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.734),
    }
}
