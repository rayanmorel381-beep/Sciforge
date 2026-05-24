use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H10",
        name: "1-hexyne",
        composition: &[(6, 6), (1, 10)],
        molar_mass: 82.145,
        category: "alkyne",
        state_at_stp: "liquid",
        melting_point_k: Some(141.3),
        boiling_point_k: Some(344.5),
        density_g_cm3: Some(0.715),
    }
}
