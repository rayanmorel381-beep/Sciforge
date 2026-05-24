use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cl2",
        name: "dichlorine",
        composition: &[(17, 2)],
        molar_mass: 70.906,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(171.6),
        boiling_point_k: Some(239.11),
        density_g_cm3: Some(0.003_2),
    }
}
