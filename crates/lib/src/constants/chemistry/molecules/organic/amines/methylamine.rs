use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3NH2",
        name: "methylamine",
        composition: &[(6, 1), (1, 5), (7, 1)],
        molar_mass: 31.058,
        category: "amine",
        state_at_stp: "gas",
        melting_point_k: Some(179.7),
        boiling_point_k: Some(266.8),
        density_g_cm3: Some(0.6562),
    }
}
