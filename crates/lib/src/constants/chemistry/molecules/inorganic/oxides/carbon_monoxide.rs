use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CO",
        name: "carbon monoxide",
        composition: &[(6, 1), (8, 1)],
        molar_mass: 28.010,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(68.13),
        boiling_point_k: Some(81.65),
        density_g_cm3: Some(0.001_145),
    }
}
