use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5SH",
        name: "ethanethiol",
        composition: &[(6, 2), (1, 6), (16, 1)],
        molar_mass: 62.135,
        category: "thiol",
        state_at_stp: "liquid",
        melting_point_k: Some(125.3),
        boiling_point_k: Some(308.1),
        density_g_cm3: Some(0.839),
    }
}
