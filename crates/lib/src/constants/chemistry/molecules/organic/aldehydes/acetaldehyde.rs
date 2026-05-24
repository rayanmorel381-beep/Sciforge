use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3CHO",
        name: "acetaldehyde",
        composition: &[(6, 2), (1, 4), (8, 1)],
        molar_mass: 44.053,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(150.0),
        boiling_point_k: Some(293.3),
        density_g_cm3: Some(0.784),
    }
}
