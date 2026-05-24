use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H8",
        name: "propane",
        composition: &[(6, 3), (1, 8)],
        molar_mass: 44.096,
        category: "alkane",
        state_at_stp: "gas",
        melting_point_k: Some(85.5),
        boiling_point_k: Some(231.05),
        density_g_cm3: Some(0.002009),
    }
}
