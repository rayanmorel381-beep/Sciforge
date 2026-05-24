use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5OH",
        name: "ethanol",
        composition: &[(6, 2), (1, 6), (8, 1)],
        molar_mass: 46.069,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(159.0),
        boiling_point_k: Some(351.5),
        density_g_cm3: Some(0.7893),
    }
}
