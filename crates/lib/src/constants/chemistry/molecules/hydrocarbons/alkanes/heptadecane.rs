use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H36",
        name: "heptadecane",
        composition: &[(6, 17), (1, 36)],
        molar_mass: 240.468,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(295.1),
        boiling_point_k: Some(575.2),
        density_g_cm3: Some(0.778),
    }
}
