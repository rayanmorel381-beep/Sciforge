use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H12N2O",
        name: "serotonin",
        composition: &[(6, 10), (1, 12), (7, 2), (8, 1)],
        molar_mass: 176.215,
        category: "neurotransmitter",
        state_at_stp: "solid",
        melting_point_k: Some(440.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.310),
    }
}
