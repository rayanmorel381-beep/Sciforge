use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6",
        name: "1-butyne",
        composition: &[(6, 4), (1, 6)],
        molar_mass: 54.092,
        category: "alkyne",
        state_at_stp: "gas",
        melting_point_k: Some(147.4),
        boiling_point_k: Some(281.2),
        density_g_cm3: Some(0.650),
    }
}
