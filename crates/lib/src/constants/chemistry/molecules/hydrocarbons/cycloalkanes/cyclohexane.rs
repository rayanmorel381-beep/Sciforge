use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12",
        name: "cyclohexane",
        composition: &[(6, 6), (1, 12)],
        molar_mass: 84.160,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(279.6),
        boiling_point_k: Some(353.87),
        density_g_cm3: Some(0.7781),
    }
}
