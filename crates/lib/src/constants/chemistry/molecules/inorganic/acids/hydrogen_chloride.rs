use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCl",
        name: "hydrogen chloride",
        composition: &[(1, 1), (17, 1)],
        molar_mass: 36.458,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(158.8),
        boiling_point_k: Some(188.0),
        density_g_cm3: Some(0.001_490),
    }
}
